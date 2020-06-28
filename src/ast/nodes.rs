use super::{Node, Rule};
use crate::ast::helpers;
use crate::error::*;
use pest::iterators::Pair;

node! {
    struct Script = Rule::script {
        stmt: Vec<Stmt>,
        expr: Seq,
    }
}

node! {
    struct Block = Rule::block {
        stmt: Vec<Stmt>,
        expr: Seq,
    }
}

node! {
    enum Stmt = Rule::stmt {
        Let(LetStmt),
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

node! {
    struct Seq = Rule::seq {
        expr: Vec<Expr>,
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    BinOp(Box<Expr>, Op, Box<Expr>),
    Call(Box<Expr>, Seq),
    Object(ObjectExpr),
    Array(ArrayExpr),
    Block(Block),
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
        helpers::log_call::<Self, _, _>("parse", move || {
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
                        let atom = if let Some(atom) = <Option<ObjectExpr>>::parse_many(&mut pairs)?
                        {
                            Expr::Object(atom)
                        } else if let Some(atom) = <Option<ArrayExpr>>::parse_many(&mut pairs)? {
                            Expr::Array(atom)
                        } else if let Some(atom) = <Option<Block>>::parse_many(&mut pairs)? {
                            Expr::Block(atom)
                        } else if let Some(atom) = <Option<StringExpr>>::parse_many(&mut pairs)? {
                            Expr::String(atom)
                        } else if let Some(atom) = <Option<NumberExpr>>::parse_many(&mut pairs)? {
                            Expr::Number(atom)
                        } else if let Some(atom) = <Option<Ident>>::parse_many(&mut pairs)? {
                            Expr::Ident(atom)
                        } else {
                            unreachable!("rule {:?}", pairs.peek().as_ref().map(Pair::as_rule))
                        };

                        // parse arguments
                        let arg_tuples: Vec<Seq> = Node::parse_many(&mut pairs)?;
                        helpers::check_end(pairs)?;

                        let mut expr = atom;
                        for arg_tuple in arg_tuples {
                            expr = Expr::Call(Box::new(expr), arg_tuple);
                        }

                        Ok(expr)
                    }

                    rule @ _ => unreachable!("rule {:?}", rule),
                },
                |lhs, op, rhs| Ok(Expr::BinOp(Box::new(lhs?), Op::parse(op)?, Box::new(rhs?))),
            )
        })
    }
}

#[derive(Clone, Debug)]
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

node! {
    struct ArrayExpr = Rule::array {
        items: Vec<Expr>,
    }
}

node! {
    struct StringExpr = Rule::string {
        content: String,
    }
}

node! {
    struct NumberExpr = Rule::number
}

node! {
    struct Ident = Rule::ident
}

node!(struct KwLet = Rule::kw_let);
