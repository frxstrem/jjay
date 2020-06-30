use crate::error::*;
use crate::scope::Scope;
use crate::value::Value;

pub fn scope(call_scope: Scope, _: Value) -> ScriptResult<Value> {
    Ok(Value::Object(
        call_scope
            .values_recurse()
            .map(|(key, value)| (key.to_string(), value.clone()))
            .collect(),
    ))
}

pub fn local_scope(call_scope: Scope, _: Value) -> ScriptResult<Value> {
    Ok(Value::Object(
        call_scope
            .values()
            .map(|(key, value)| (key.to_string(), value.clone()))
            .collect(),
    ))
}

pub fn pipe(call_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
    rhs.invoke(call_scope, lhs)
}

pub fn add(_call_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
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

pub fn sub(_call_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
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

pub fn mul(_call_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
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

pub fn div(_call_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
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

pub fn eq(_call_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
    Err(script_error!(
        "comparison not implement for types: {:?}, {:?}",
        lhs.value_type(),
        rhs.value_type()
    ))
}

pub fn ne(_call_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
    Err(script_error!(
        "comparison not implement for types: {:?}, {:?}",
        lhs.value_type(),
        rhs.value_type()
    ))
}

pub fn ge(_call_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
    Err(script_error!(
        "comparison not implement for types: {:?}, {:?}",
        lhs.value_type(),
        rhs.value_type()
    ))
}

pub fn le(_call_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
    Err(script_error!(
        "comparison not implement for types: {:?}, {:?}",
        lhs.value_type(),
        rhs.value_type()
    ))
}

pub fn gt(_call_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
    Err(script_error!(
        "comparison not implement for types: {:?}, {:?}",
        lhs.value_type(),
        rhs.value_type()
    ))
}

pub fn lt(_call_scope: Scope, lhs: Value, rhs: Value) -> ScriptResult<Value> {
    Err(script_error!(
        "comparison not implement for types: {:?}, {:?}",
        lhs.value_type(),
        rhs.value_type()
    ))
}
