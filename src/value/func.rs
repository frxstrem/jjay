use std::fmt::{self, Debug};
use std::sync::Arc;

use crate::ast::{Expr, FnArg};
use crate::error::*;
use crate::eval::Evaluate;
use crate::scope::Scope;
use crate::value::Value;

#[derive(Clone, Debug)]
pub enum Function {
    Code(Scope, Option<FnArg>, Box<Expr>),
    Nested(Scope, Option<FnArg>, Arc<Function>),
    Native(NativeFunction),
}

impl Function {
    pub fn invoke(&self, call_scope: Scope, arg: Value) -> ScriptResult<Value> {
        match self {
            Function::Code(scope, fn_arg, expr) => {
                let mut scope = scope.clone();
                if let Some(fn_arg) = fn_arg {
                    scope = scope.set(&fn_arg.name.value, arg)?;
                }
                expr.evaluate_value(scope)
            }
            Function::Nested(scope, fn_arg, func) => {
                let mut scope = scope.clone();
                if let Some(fn_arg) = fn_arg {
                    scope = scope.set(&fn_arg.name.value, arg)?;
                }
                Ok(func.extend_scope(scope).into())
            }
            Function::Native(func) => (func.0)(call_scope, arg),
        }
    }

    pub fn new_from_expr(scope: &Scope, arg: Option<FnArg>, expr: Expr) -> Function {
        Function::Code(scope.inherit(), arg, Box::new(expr))
    }

    pub fn new_nested(scope: &Scope, arg: Option<FnArg>, nested: Function) -> Function {
        Function::Nested(scope.inherit(), arg, Arc::new(nested))
    }

    fn extend_scope(&self, scope: Scope) -> Function {
        match self {
            Function::Code(old_scope, fn_arg, expr) => Function::Code(
                old_scope.clone().extend(scope),
                fn_arg.clone(),
                expr.clone(),
            ),
            Function::Nested(old_scope, fn_arg, func) => Function::Nested(
                old_scope.clone().extend(scope),
                fn_arg.clone(),
                func.clone(),
            ),
            Function::Native(func) => Function::Native(func.clone()),
        }
    }

    pub fn new<F>(f: F) -> Function
    where
        F: 'static + Fn(Scope, Value) -> ScriptResult<Value>,
    {
        Function::Native(NativeFunction(Arc::new(f)))
    }

    pub fn new2<F>(f: F) -> Function
    where
        F: 'static + Fn(Scope, Value, Value) -> ScriptResult<Value>,
    {
        let f = Arc::new(f);
        Function::new(move |_: Scope, arg0: Value| {
            let f = f.clone();
            Ok(Function::new(move |call_scope, arg1| f(call_scope, arg0.clone(), arg1)).into())
        })
    }

    pub fn new3<F>(f: F) -> Function
    where
        F: 'static + Fn(Scope, Value, Value, Value) -> ScriptResult<Value>,
    {
        let f = Arc::new(f);
        Function::new(move |_: Scope, arg0: Value| {
            let f = f.clone();
            Ok(Function::new(move |_, arg1| {
                let f = f.clone();
                let arg0 = arg0.clone();
                Ok(Function::new(move |call_scope, arg2| {
                    f(call_scope, arg0.clone(), arg1.clone(), arg2)
                })
                .into())
            })
            .into())
        })
    }
}

impl From<Function> for Value {
    fn from(func: Function) -> Value {
        Value::Function(func)
    }
}

#[derive(Clone)]
pub struct NativeFunction(Arc<dyn Fn(Scope, Value) -> ScriptResult<Value>>);

impl Debug for NativeFunction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "NativeFunction(...)")
    }
}
