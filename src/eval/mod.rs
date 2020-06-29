mod func;
mod scope;
mod value;

use crate::error::*;

pub use self::func::Function;
pub use self::scope::*;
pub use self::value::*;

pub trait Evaluate {
    fn evaluate(&self, scope: Scope) -> ScriptResult<(Scope, Value)>;

    fn evaluate_value(&self, scope: Scope) -> ScriptResult<Value> {
        self.evaluate(scope).map(|(_, value)| value)
    }
}
