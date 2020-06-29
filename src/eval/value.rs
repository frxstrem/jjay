use std::collections::BTreeMap;
use std::fmt::{self, Debug, Display};

use super::{Function, Scope};
use crate::error::*;

#[derive(Clone, Debug)]
pub enum Value {
    Object(BTreeMap<String, Value>),
    Array(Vec<Value>),
    Number(f64),
    String(String),
    Boolean(bool),
    Null,

    Function(Function),
}

impl Value {
    pub fn new_object<I>(value: I) -> Value
    where
        I: IntoIterator<Item = (String, Value)>,
    {
        Value::Object(value.into_iter().collect())
    }

    pub fn new_array<I>(value: I) -> Value
    where
        I: IntoIterator<Item = Value>,
    {
        Value::Array(value.into_iter().collect())
    }

    pub const fn new_number(value: f64) -> Value {
        Value::Number(value)
    }

    pub const fn new_string(value: String) -> Value {
        Value::String(value)
    }

    pub const fn new_bool(value: bool) -> Value {
        Value::Boolean(value)
    }

    pub const fn new_null() -> Value {
        Value::Null
    }

    pub fn invoke(&self, scope: Scope, arg: Value) -> ScriptResult<Value> {
        match self {
            Value::Function(func) => func.invoke(scope, arg),
            value @ _ => Err(ScriptError::NotCallable(value.value_type())),
        }
    }

    pub fn to_string(&self) -> ScriptResult<String> {
        match self {
            Value::Number(value) => Ok(format!("{}", value)),
            Value::String(value) => Ok(format!("{}", value)),
            Value::Boolean(true) => Ok(format!("true")),
            Value::Boolean(false) => Ok(format!("false")),
            Value::Null => Ok(format!("null")),

            value @ _ => Err(ScriptError::NotStringConvertible(value.value_type())),
        }
    }

    fn to_json_opt(&self) -> ScriptResult<Option<serde_json::Value>> {
        match self {
            Value::Object(map) => Ok(Some(serde_json::Value::Object(
                map.iter()
                    .map(|(key, value)| Ok((key.clone(), value.to_json_opt()?)))
                    .filter_map(|entry| match entry {
                        Ok((key, Some(value))) => Some(Ok((key, value))),
                        Ok((_, None)) => None,
                        Err(err) => Some(Err(err)),
                    })
                    .collect::<ScriptResult<_>>()?,
            ))),

            Value::Array(array) => Ok(Some(serde_json::Value::Array(
                array
                    .iter()
                    .map(|value| value.to_json_opt())
                    .filter_map(|value| value.transpose())
                    .collect::<ScriptResult<_>>()?,
            ))),

            Value::Number(number) => Ok(Some(
                serde_json::Number::from_f64(*number)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null),
            )), // TODO: handle out-of-range numbers properly...

            Value::String(string) => Ok(Some(serde_json::Value::String(string.clone()))),

            Value::Boolean(b) => Ok(Some(serde_json::Value::Bool(*b))),

            Value::Null => Ok(Some(serde_json::Value::Null)),

            Value::Function(..) => Ok(None),
        }
    }

    pub fn to_json(&self) -> ScriptResult<serde_json::Value> {
        Ok(self.to_json_opt()?.unwrap_or(serde_json::Value::Null))
    }

    pub fn write_to<W: std::io::Write>(&self, mut writer: W) -> ScriptResult<()> {
        let value = self.to_json()?;
        serde_json::to_writer(&mut writer, &value).map_err(std::io::Error::from)?;
        writeln!(writer)?;
        Ok(())
    }

    pub fn write_to_pretty<W: std::io::Write>(&self, mut writer: W) -> ScriptResult<()> {
        let value = self.to_json()?;
        serde_json::to_writer_pretty(&mut writer, &value).map_err(std::io::Error::from)?;
        writeln!(writer)?;
        Ok(())
    }

    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Object(..) => ValueType::Object,
            Value::Array(..) => ValueType::Array,
            Value::Number(..) => ValueType::Number,
            Value::String(..) => ValueType::String,
            Value::Boolean(..) => ValueType::Boolean,
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
