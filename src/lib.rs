#[macro_use]
mod macros;

#[macro_use]
pub mod error;

mod ast;
mod eval;
mod scope;
mod value;

use crate::eval::Evaluate;

pub use crate::error::*;
pub use crate::scope::Scope;
pub use crate::value::Value;

pub fn run_script(source: impl AsRef<str>) -> ScriptResult<Value> {
    let scope = Scope::new_default();
    run_script_with(source, scope)
}

pub fn run_script_with(source: impl AsRef<str>, scope: Scope) -> ScriptResult<Value> {
    // parse script
    let script = ast::parse_str(source.as_ref())?;

    // evaluate script
    let value = script.evaluate_value(scope)?;

    Ok(value)
}
