use pest::iterators::Pair;
use std::collections::BTreeMap;

use super::{Node, Rule};
use crate::ast::helpers;
use crate::error::*;
use crate::eval::Evaluate;
use crate::scope::Scope;
use crate::value::{Function, Value};

node! {
    struct Script = Rule::script {
        stmts: Vec<Stmt>,
        expr: Expr,
    }
}

impl Evaluate for Script {
    fn evaluate(&self, mut scope: Scope) -> ScriptResult<(Scope, Value)> {
        for stmt in &self.stmts {
            let (s, _) = stmt.evaluate(scope)?;
            scope = s;
        }

        self.expr.evaluate(scope)
    }
}

node! {
    struct Block = Rule::block {
        stmts: Vec<Stmt>,
        expr: Expr,
    }
}

impl Evaluate for Block {
    fn evaluate(&self, scope: Scope) -> ScriptResult<(Scope, Value)> {
        let mut scope = scope.inherit();

        for stmt in &self.stmts {
            let (s, _) = stmt.evaluate(scope)?;
            scope = s;
        }

        let (scope, value) = self.expr.evaluate(scope.clone())?;
        Ok((scope, value.simplify()))
    }
}

node! {
    enum Stmt = Rule::stmt {
        Let(LetStmt),
    }
}

impl Evaluate for Stmt {
    fn evaluate(&self, scope: Scope) -> ScriptResult<(Scope, Value)> {
        match self {
            Stmt::Let(inner) => inner.evaluate(scope),
        }
    }
}

node! {
    struct LetStmt = Rule::let_stmt {
        let_: KwLet,
        name: Ident,
        args: Vec<FnArgs>,
        value: Expr,
    }
}

impl Evaluate for LetStmt {
    fn evaluate(&self, scope: Scope) -> ScriptResult<(Scope, Value)> {
        let value = if self.args.is_empty() {
            self.value.evaluate_value(scope.clone())?
        } else {
            let mut args: Vec<_> = self.args.iter().collect();

            let arg = args.pop().unwrap();
            let mut func = Function::new_from_expr(&scope, arg.arg.clone(), self.value.clone());

            while let Some(arg) = args.pop() {
                func = Function::new_nested(&scope, arg.arg.clone(), func);
            }

            func.into()
        };

        let scope = scope.set(&self.name.value, value)?;
        Ok((scope, Value::Null))
    }
}

node! {
    struct FnArgs = Rule::fn_args {
        arg: Option<FnArg>,
    }
}

node! {
    struct FnArg = Rule::fn_arg {
        name: Ident,
        ty: Option<Expr>,
    }
}

impl FnArg {
    pub fn from_ident(ident: Ident) -> FnArg {
        FnArg {
            name: ident,
            ty: None,
        }
    }
}

node! {
    struct Seq = Rule::seq {
        exprs: Vec<Expr>,
    }
}

impl From<Vec<Expr>> for Seq {
    fn from(exprs: Vec<Expr>) -> Seq {
        Seq { exprs }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    BinOp(Box<Expr>, Op, Box<Expr>),
    Call(Box<Expr>, ArgList),
    PathAccess(Box<Expr>, PathSegment, Option<NullPropagation>),
    NullPropagate(Box<Expr>),
    Object(ObjectExpr),
    Array(ArrayExpr),
    Lambda(LambdaExpr),
    Block(Box<Block>),
    String(StringExpr),
    Number(NumberExpr),
    Ident(Ident),
}

impl Node for Expr {
    fn can_parse(rule: &Rule) -> bool {
        rule == &Rule::expr
    }

    fn parse(pair: Pair<Rule>) -> ParseResult<Self> {
        use pest::prec_climber::{Assoc, Operator, PrecClimber};

        helpers::check_rule(&pair, &Rule::expr)?;

        let prec_climber = PrecClimber::new(vec![
            Operator::new(Rule::pipe, Assoc::Left),
            Operator::new(Rule::eq, Assoc::Left)
                | Operator::new(Rule::ne, Assoc::Left)
                | Operator::new(Rule::ge, Assoc::Left)
                | Operator::new(Rule::le, Assoc::Left)
                | Operator::new(Rule::gt, Assoc::Left)
                | Operator::new(Rule::lt, Assoc::Left),
            Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::sub, Assoc::Left),
            Operator::new(Rule::mul, Assoc::Left) | Operator::new(Rule::div, Assoc::Left),
        ]);

        prec_climber.climb(
            pair.into_inner(),
            |pair| match pair.as_rule() {
                Rule::expr_call => {
                    let mut pairs = pair.into_inner();

                    // try to parse expression atom
                    let atom = if let Some(atom) = <Option<ObjectExpr>>::parse_many(&mut pairs)? {
                        Expr::Object(atom)
                    } else if let Some(atom) = <Option<ArrayExpr>>::parse_many(&mut pairs)? {
                        Expr::Array(atom)
                    } else if let Some(atom) = <Option<LambdaExpr>>::parse_many(&mut pairs)? {
                        Expr::Lambda(atom)
                    } else if let Some(atom) = <Option<Block>>::parse_many(&mut pairs)? {
                        Expr::Block(Box::new(atom))
                    } else if let Some(atom) = <Option<StringExpr>>::parse_many(&mut pairs)? {
                        Expr::String(atom)
                    } else if let Some(atom) = <Option<NumberExpr>>::parse_many(&mut pairs)? {
                        Expr::Number(atom)
                    } else if let Some(atom) = <Option<Ident>>::parse_many(&mut pairs)? {
                        Expr::Ident(atom)
                    } else {
                        unreachable!("rule {:?}", pairs.peek().as_ref().map(Pair::as_rule));
                    };

                    // parse null propagation, path segments and function argument
                    let mut expr = atom;
                    while pairs.peek().is_some() {
                        if let Some(_) = <Option<NullPropagation>>::parse_many(&mut pairs)? {
                            expr = Expr::NullPropagate(Box::new(expr));
                        } else if let Some(path_segment) =
                            <Option<PathSegment>>::parse_many(&mut pairs)?
                        {
                            let null_propagation = Node::parse_many(&mut pairs)?;
                            expr = Expr::PathAccess(Box::new(expr), path_segment, null_propagation);
                        } else if let Some(arg_list) = <Option<ArgList>>::parse_many(&mut pairs)? {
                            expr = Expr::Call(Box::new(expr), arg_list);
                        } else {
                            unreachable!("rule {:?}", pairs.peek().as_ref().map(Pair::as_rule));
                        }
                    }

                    // let mut expr = atom;
                    // for arg_list in arg_lists {
                    //     expr = Expr::Call(Box::new(expr), arg_list);
                    // }

                    Ok(expr)
                }

                rule @ _ => unreachable!("rule {:?}", rule),
            },
            |lhs, op, rhs| Ok(Expr::BinOp(Box::new(lhs?), Op::parse(op)?, Box::new(rhs?))),
        )
    }
}

impl Evaluate for Expr {
    fn evaluate(&self, scope: Scope) -> ScriptResult<(Scope, Value)> {
        let value = match self {
            Expr::BinOp(lhs, op, rhs) => {
                let lhs = lhs.evaluate_value(scope.clone())?;
                let rhs = rhs.evaluate_value(scope.clone())?;
                let func_name = op.func_name();

                let func = scope.get(func_name)?;
                let func = evaluate_func_call(scope.clone(), func, lhs)?;
                evaluate_func_call(scope.clone(), func, rhs)?
            }

            Expr::Call(func, args) => {
                let func = func.evaluate_value(scope.clone())?;
                let arg = args
                    .arg
                    .as_ref()
                    .map(|expr| expr.evaluate_value(scope.clone()))
                    .transpose()?
                    .unwrap_or(Value::Null);
                evaluate_func_call(scope.clone(), func, arg)?
            }

            Expr::PathAccess(expr, path_segment, null_propagation) => {
                let value = expr.evaluate_value(scope.clone())?;
                let key = path_segment.evaluate_value(scope.clone())?;
                value.get_property(scope.clone(), key, null_propagation.is_some())?
            }

            Expr::NullPropagate(expr) => {
                let value = expr.evaluate_value(scope.clone())?;
                value.or_propagated_null()
            }

            Expr::Object(object) => {
                let mut properties = BTreeMap::new();

                for entry in &object.entries {
                    let key = entry.key.evaluate_value(scope.clone())?.to_string()?;
                    let value = entry.value.evaluate_value(scope.clone())?;

                    properties.insert(key, value);
                }

                Value::Object(properties)
            }

            Expr::Array(array) => Value::Array(
                array
                    .items
                    .iter()
                    .map(|item| item.evaluate_value(scope.clone()))
                    .collect::<ScriptResult<_>>()?,
            ),

            Expr::Lambda(lambda) => lambda.evaluate_value(scope.clone())?,

            Expr::Block(block) => block.evaluate_value(scope.clone())?,

            Expr::Number(number) => number.decode().map(Value::Number)?,
            Expr::String(string) => string.decode().map(Value::String)?,

            Expr::Ident(ident) => scope.get(&ident.value)?,
        };
        Ok((scope, value))
    }
}

fn evaluate_func_call(call_scope: Scope, func: Value, arg: Value) -> ScriptResult<Value> {
    let value = func.invoke(call_scope, arg)?;
    Ok(value)
}

#[derive(Copy, Clone, Debug)]
pub enum Op {
    Pipe,
    Eq,
    Ne,
    Le,
    Ge,
    Lt,
    Gt,
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn func_name(&self) -> &'static str {
        match self {
            Op::Pipe => "/pipe",
            Op::Eq => "/eq",
            Op::Ne => "/ne",
            Op::Le => "/le",
            Op::Ge => "/ge",
            Op::Lt => "/lt",
            Op::Gt => "/gt",
            Op::Add => "/add",
            Op::Sub => "/sub",
            Op::Mul => "/mul",
            Op::Div => "/div",
        }
    }
}

impl Node for Op {
    fn can_parse(rule: &Rule) -> bool {
        match rule {
            Rule::pipe
            | Rule::eq
            | Rule::ne
            | Rule::le
            | Rule::ge
            | Rule::lt
            | Rule::gt
            | Rule::add
            | Rule::sub
            | Rule::mul
            | Rule::div => true,
            _ => false,
        }
    }

    fn parse(pair: Pair<Rule>) -> ParseResult<Self> {
        Ok(match pair.as_rule() {
            Rule::pipe => Op::Pipe,
            Rule::eq => Op::Eq,
            Rule::ne => Op::Ne,
            Rule::le => Op::Le,
            Rule::ge => Op::Ge,
            Rule::lt => Op::Lt,
            Rule::gt => Op::Gt,
            Rule::add => Op::Add,
            Rule::sub => Op::Sub,
            Rule::mul => Op::Mul,
            Rule::div => Op::Div,

            rule @ _ => unreachable!("rule {:?}", rule),
        })
    }
}

node! {
    struct ArgList = Rule::args {
        arg: Option<Box<Expr>>,
    }
}

node! {
    enum PathSegment = Rule::path_segment {
        Ident(Ident),
        Expr(Box<Expr>),
    }
}

impl Evaluate for PathSegment {
    fn evaluate(&self, scope: Scope) -> ScriptResult<(Scope, Value)> {
        let value = match self {
            Self::Ident(ident) => Value::String(ident.value.clone()),
            Self::Expr(expr) => expr.evaluate_value(scope.clone())?,
        };
        Ok((scope, value))
    }
}

node! {
    struct NullPropagation = Rule::null_propagation
}

node! {
    struct ObjectExpr = Rule::object {
        entries: Vec<ObjectEntry>,
    }
}

node! {
    struct ObjectEntry = Rule::object_entry {
        key: ObjectKey,
        value: Expr,
    }
}

node! {
    enum ObjectKey = Rule::object_key {
        String(StringExpr),
        Ident(Ident),
    }
}

impl Evaluate for ObjectKey {
    fn evaluate(&self, scope: Scope) -> ScriptResult<(Scope, Value)> {
        match self {
            ObjectKey::String(string) => string.evaluate(scope),
            ObjectKey::Ident(ident) => Ok((scope, Value::String(ident.value.clone()))),
        }
    }
}

node! {
    struct ArrayExpr = Rule::array {
        items: Vec<Expr>,
    }
}

node! {
    struct LambdaExpr = Rule::lambda {
        args: Vec<Ident>,
        expr: Box<Expr>,
    }
}

impl Evaluate for LambdaExpr {
    fn evaluate(&self, scope: Scope) -> ScriptResult<(Scope, Value)> {
        let mut args: Vec<_> = self.args.iter().collect();

        let arg = args.pop().unwrap();
        let mut func = Function::new_from_expr(
            &scope,
            Some(FnArg::from_ident(arg.clone())),
            *self.expr.clone(),
        );

        while let Some(arg) = args.pop() {
            func = Function::new_nested(&scope, Some(FnArg::from_ident(arg.clone())), func);
        }

        Ok((scope, func.into()))
    }
}

node! {
    struct StringExpr = Rule::string {
        value: String,
    }
}

impl StringExpr {
    pub fn decode(&self) -> ScriptResult<String> {
        serde_json::from_str(&format!("\"{}\"", self.value))
            .map_err(|_| ScriptError::Other(format!("Invalid string literal: \"{}\"", self.value)))
    }
}

impl Evaluate for StringExpr {
    fn evaluate(&self, scope: Scope) -> ScriptResult<(Scope, Value)> {
        let value = self.decode()?;
        Ok((scope, Value::String(value)))
    }
}

node! {
    struct NumberExpr = Rule::number
}

impl NumberExpr {
    pub fn decode(&self) -> ScriptResult<f64> {
        serde_json::from_str(&self.value)
            .map_err(|_| ScriptError::Other(format!("Invalid numeric literal: {}", self.value)))
    }
}

impl Evaluate for NumberExpr {
    fn evaluate(&self, scope: Scope) -> ScriptResult<(Scope, Value)> {
        let value = self.decode()?;
        Ok((scope, Value::Number(value)))
    }
}

node! {
    struct Ident = Rule::ident
}

node!(struct KwLet = Rule::kw_let);
