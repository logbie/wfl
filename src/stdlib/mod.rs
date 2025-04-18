pub mod core;
pub mod math;
pub mod text;
pub mod list;
pub mod typechecker;

use crate::interpreter::environment::Environment;

pub fn register_stdlib(env: &mut Environment) {
    core::register_core(env);
    math::register_math(env);
    text::register_text(env);
    list::register_list(env);
}
