mod stdlib;

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use crate::error::*;
use crate::value::{Function, Value};

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
            .set_nofail("scope", Function::new(stdlib::scope))
            .set_nofail("local_scope", Function::new(stdlib::local_scope))
            .set_nofail("/pipe", Function::new2(stdlib::pipe))
            .set_nofail("/add", Function::new2(stdlib::add))
            .set_nofail("/sub", Function::new2(stdlib::sub))
            .set_nofail("/mul", Function::new2(stdlib::mul))
            .set_nofail("/div", Function::new2(stdlib::div))
            .set_nofail("/eq", Function::new2(stdlib::eq))
            .set_nofail("/ne", Function::new2(stdlib::ne))
            .set_nofail("/ge", Function::new2(stdlib::ge))
            .set_nofail("/le", Function::new2(stdlib::le))
            .set_nofail("/gt", Function::new2(stdlib::gt))
            .set_nofail("/lt", Function::new2(stdlib::lt))
            .inherit()
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
        self.values
            .insert(name.to_string(), value.into().simplify());
        self
    }

    pub fn extend(self, other: Scope) -> Scope {
        let mut scope = self;
        for (name, value) in other.values {
            scope = scope.set_nofail(&name, value);
        }
        scope
    }

    pub fn values(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.values.iter().map(|(key, value)| (key.as_str(), value))
    }

    pub fn values_recurse(&self) -> impl Iterator<Item = (&str, &Value)> {
        if let Some(parent) = &self.parent {
            Box::new(self.values().chain(parent.values_recurse()))
                as Box<dyn Iterator<Item = (&str, &Value)>>
        } else {
            Box::new(self.values())
        }
    }
}
