use std::collections::HashMap;
use std::fmt::{self, Debug, Display};

use super::{Function, Scope};
use crate::error::*;

#[derive(Clone, Debug)]
pub enum Value {
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Number(f64),
    String(String),
    True,
    False,
    Null,

    Function(Function),
}

impl Value {
    pub fn invoke(&self, scope: Scope, arg: Value) -> ScriptResult<Value> {
        match self {
            Value::Function(func) => func.invoke(scope, arg),
            _ => Err(ScriptError::NotCallable),
        }
    }

    pub fn to_string(&self) -> ScriptResult<String> {
        match self {
            Value::Number(value) => Ok(format!("{}", value)),
            Value::String(value) => Ok(format!("{}", value)),
            Value::True => Ok(format!("true")),
            Value::False => Ok(format!("false")),
            Value::Null => Ok(format!("null")),

            _ => Err(ScriptError::NotStringConvertible),
        }
    }

    pub fn to_json(&self) -> ScriptResult<serde_json::Value> {
        todo!()
    }

    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Object(..) => ValueType::Object,
            Value::Array(..) => ValueType::Array,
            Value::Number(..) => ValueType::Number,
            Value::String(..) => ValueType::String,
            Value::True | Value::False => ValueType::Boolean,
            Value::Null => ValueType::Null,
            Value::Function(..) => ValueType::Function,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ValueType {
    Object,
    Array,
    Number,
    String,
    Boolean,
    Null,
    Function,
}

impl Display for ValueType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}
