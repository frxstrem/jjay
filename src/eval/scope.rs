use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use super::{Function, Value};
use crate::error::*;

#[derive(Clone, Debug)]
pub struct Scope {
    parent: Option<Arc<Scope>>,
    values: HashMap<String, Value>,
}

impl Scope {
    pub fn new_empty() -> Scope {
        Scope {
            parent: None,
            values: HashMap::new(),
        }
    }

    pub fn new_default() -> Scope {
        Scope::new_empty()
            .set_nofail("true", Value::Boolean(true))
            .set_nofail("false", Value::Boolean(false))
            .set_nofail("null", Value::Null)
            .set_nofail("/pipe", Function::new2(default_fns::pipe))
            .set_nofail("/add", Function::new2(default_fns::add))
            .set_nofail("/sub", Function::new2(default_fns::sub))
            .set_nofail("/mul", Function::new2(default_fns::mul))
            .set_nofail("/div", Function::new2(default_fns::div))
            .set_nofail("/eq", Function::new2(default_fns::eq))
            .set_nofail("/ne", Function::new2(default_fns::ne))
            .set_nofail("/ge", Function::new2(default_fns::ge))
            .set_nofail("/le", Function::new2(default_fns::le))
            .set_nofail("/gt", Function::new2(default_fns::gt))
            .set_nofail("/lt", Function::new2(default_fns::lt))
    }

    pub fn inherit(&self) -> Scope {
        Scope {
            parent: Some(Arc::new(self.clone())),
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> ScriptResult<Value> {
        if let Some(value) = self.values.get(name) {
            Ok(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            Err(ScriptError::VariableNotFound(name.to_string()))
        }
    }

    pub fn set(self, name: &str, value: impl Into<Value>) -> ScriptResult<Scope> {
        if self.values.contains_key(name) {
            return Err(ScriptError::VariableAlreadyExists(name.to_string()));
        }

        Ok(self.set_nofail(name, value))
    }

    pub fn set_nofail(mut self, name: &str, value: impl Into<Value>) -> Scope {
        self.values.insert(name.to_string(), value.into());
        self
    }

    pub fn extend(self, other: Scope) -> Scope {
        let mut scope = self;
        for (name, value) in other.values {
            scope = scope.set_nofail(&name, value);
        }
        scope
    }
}

mod default_fns {
    use super::*;

    pub(crate) fn pipe(lhs: Value, rhs: Value) -> ScriptResult<Value> {
        rhs.invoke(lhs)
    }

    pub(crate) fn add(lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Ok(match (&lhs, &rhs) {
            (Value::Number(x), Value::Number(y)) => Value::Number(x + y),

            _ => {
                return Err(script_error!(
                    "cannot add values of types: {:?}, {:?}",
                    lhs.value_type(),
                    rhs.value_type()
                ))
            }
        })
    }

    pub(crate) fn sub(lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Ok(match (&lhs, &rhs) {
            (Value::Number(x), Value::Number(y)) => Value::Number(x - y), // TODO: checked

            _ => {
                return Err(script_error!(
                    "cannot subtract values of types: {:?}, {:?}",
                    lhs.value_type(),
                    rhs.value_type()
                ))
            }
        })
    }

    pub(crate) fn mul(lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Ok(match (&lhs, &rhs) {
            (Value::Number(x), Value::Number(y)) => Value::Number(x * y), // TODO: checked

            _ => {
                return Err(script_error!(
                    "cannot multiply values of types: {:?}, {:?}",
                    lhs.value_type(),
                    rhs.value_type()
                ))
            }
        })
    }

    pub(crate) fn div(lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Ok(match (&lhs, &rhs) {
            (Value::Number(x), Value::Number(y)) => Value::Number(x / y), // TODO: checked

            _ => {
                return Err(script_error!(
                    "cannot divide values of types: {:?}, {:?}",
                    lhs.value_type(),
                    rhs.value_type()
                ))
            }
        })
    }

    pub(crate) fn eq(lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Err(script_error!(
            "comparison not implement for types: {:?}, {:?}",
            lhs.value_type(),
            rhs.value_type()
        ))
    }

    pub(crate) fn ne(lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Err(script_error!(
            "comparison not implement for types: {:?}, {:?}",
            lhs.value_type(),
            rhs.value_type()
        ))
    }

    pub(crate) fn ge(lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Err(script_error!(
            "comparison not implement for types: {:?}, {:?}",
            lhs.value_type(),
            rhs.value_type()
        ))
    }

    pub(crate) fn le(lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Err(script_error!(
            "comparison not implement for types: {:?}, {:?}",
            lhs.value_type(),
            rhs.value_type()
        ))
    }

    pub(crate) fn gt(lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Err(script_error!(
            "comparison not implement for types: {:?}, {:?}",
            lhs.value_type(),
            rhs.value_type()
        ))
    }

    pub(crate) fn lt(lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Err(script_error!(
            "comparison not implement for types: {:?}, {:?}",
            lhs.value_type(),
            rhs.value_type()
        ))
    }
}
