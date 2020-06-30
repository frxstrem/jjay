use crate::error::*;
use crate::scope::Scope;
use crate::value::Value;

pub trait Evaluate {
    fn evaluate(&self, scope: Scope) -> ScriptResult<(Scope, Value)>;

    fn evaluate_value(&self, scope: Scope) -> ScriptResult<Value> {
        self.evaluate(scope).map(|(_, value)| value)
    }
}
