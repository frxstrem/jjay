use std::collections::HashMap;
use std::fmt::{self, Debug};

use super::{Function, Value};
use crate::error::*;

#[derive(Clone, Debug)]
pub struct Scope {
    values: HashMap<String, Value>,
}

impl Scope {
    pub fn new_empty() -> Scope {
        Scope {
            values: HashMap::new(),
        }
    }

    pub fn new_default() -> ScriptResult<Scope> {
        let scope = Scope::new_empty()
            .set("/pipe", Function::new2(default_fns::pipe))?
            .set("/add", Function::new2(default_fns::add))?
            .set("/sub", Function::new2(default_fns::sub))?
            .set("/mul", Function::new2(default_fns::mul))?
            .set("/div", Function::new2(default_fns::div))?
            .set("/eq", Function::new2(default_fns::eq))?
            .set("/ne", Function::new2(default_fns::ne))?
            .set("/ge", Function::new2(default_fns::ge))?
            .set("/le", Function::new2(default_fns::le))?
            .set("/gt", Function::new2(default_fns::gt))?
            .set("/lt", Function::new2(default_fns::lt))?;
        Ok(scope)
    }

    pub fn get(&self, name: &str) -> ScriptResult<Value> {
        if let Some(value) = self.values.get(name) {
            Ok(value.clone())
        } else {
            Err(ScriptError::VariableNotFound(name.to_string()))
        }
    }

    pub fn set(mut self, name: &str, value: impl Into<Value>) -> ScriptResult<Scope> {
        if self.values.contains_key(name) {
            return Err(ScriptError::VariableAlreadyExists(name.to_string()));
        }

        self.values.insert(name.to_string(), value.into());
        Ok(self)
    }
}

mod default_fns {
    use super::*;

    pub(crate) fn pipe(scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
        rhs.invoke(scope, lhs)
    }

    pub(crate) fn add(_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
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

    pub(crate) fn sub(_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
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

    pub(crate) fn mul(_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
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

    pub(crate) fn div(_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
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

    pub(crate) fn eq(_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Err(script_error!(
            "comparison not implement for types: {:?}, {:?}",
            lhs.value_type(),
            rhs.value_type()
        ))
    }

    pub(crate) fn ne(_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Err(script_error!(
            "comparison not implement for types: {:?}, {:?}",
            lhs.value_type(),
            rhs.value_type()
        ))
    }

    pub(crate) fn ge(_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Err(script_error!(
            "comparison not implement for types: {:?}, {:?}",
            lhs.value_type(),
            rhs.value_type()
        ))
    }

    pub(crate) fn le(_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Err(script_error!(
            "comparison not implement for types: {:?}, {:?}",
            lhs.value_type(),
            rhs.value_type()
        ))
    }

    pub(crate) fn gt(_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Err(script_error!(
            "comparison not implement for types: {:?}, {:?}",
            lhs.value_type(),
            rhs.value_type()
        ))
    }

    pub(crate) fn lt(_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
        Err(script_error!(
            "comparison not implement for types: {:?}, {:?}",
            lhs.value_type(),
            rhs.value_type()
        ))
    }
}
