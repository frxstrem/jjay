use std::fmt::{self, Debug};
use std::sync::Arc;

use super::{Evaluate, Scope, Value};
use crate::ast::{Expr, FnArg};
use crate::error::*;

#[derive(Clone, Debug)]
pub enum Function {
    Code(Option<FnArg>, Expr),
    Nested(Option<FnArg>, Arc<Function>),
    Bound(Scope, Arc<Function>),
    Native(NativeFunction),
}

impl Function {
    pub fn invoke(&self, scope: Scope, arg: Value) -> ScriptResult<Value> {
        match self {
            Function::Code(fn_arg, expr) => {
                let mut scope = scope;
                if let Some(fn_arg) = fn_arg {
                    scope = scope.set(&fn_arg.name.value, arg)?;
                }
                expr.evaluate_value(scope)
            }
            Function::Nested(fn_arg, func) => {
                let mut scope = Scope::new_empty();
                if let Some(fn_arg) = fn_arg {
                    scope = scope.set(&fn_arg.name.value, arg)?;
                }
                Ok(Function::Bound(scope, func.clone()).into())
            }
            Function::Bound(bound_scope, func) => {
                let scope = scope.extend(bound_scope.clone());
                func.invoke(scope, arg)
            }
            Function::Native(func) => (func.0)(scope, arg),
        }
    }

    pub fn new_from_expr(arg: Option<FnArg>, expr: Expr) -> Function {
        Function::Code(arg, expr)
    }

    pub fn new_nested(arg: Option<FnArg>, nested: Function) -> Function {
        Function::Nested(arg, Arc::new(nested))
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
            Ok(Function::new(move |scope, arg1| f(scope, arg0.clone(), arg1)).into())
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
                Ok(
                    Function::new(move |scope, arg2| f(scope, arg0.clone(), arg1.clone(), arg2))
                        .into(),
                )
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
