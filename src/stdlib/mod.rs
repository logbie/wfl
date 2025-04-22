pub mod core;
pub mod list;
pub mod math;
pub mod pattern;
pub mod text;
pub mod typechecker;

use crate::interpreter::environment::Environment;
use crate::interpreter::Interpreter;

pub fn register_stdlib(env: &mut Environment, interpreter: &Interpreter) {
    core::register_core(env);
    math::register_math(env);
    text::register_text(env);
    list::register_list(env);
    pattern::register(env, interpreter);
}
