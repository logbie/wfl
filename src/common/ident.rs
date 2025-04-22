use std::sync::Arc;
use std::fmt;
use std::ops::Deref;

pub type Ident = Arc<str>;

pub fn display_ident(ident: &Ident, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(ident.deref())
}

pub trait IdentDisplay {
    fn fmt_ident(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl IdentDisplay for Ident {
    fn fmt_ident(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_ident(self, f)
    }
}

#[cfg(test)]
mod _ident_safety_check {
    use super::*;
    const _IS_ARC: () = {
        fn assert_arc(_: &Ident) {}
        fn _check() {
            use std::any::TypeId;
            assert_ne!(TypeId::of::<Ident>(), TypeId::of::<String>());
        }
    };
}
