use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(Arc<str>);

impl Ident {
    pub fn as_str(&self) -> &str {
        &self.0
    }

}

impl Deref for Ident {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Ident {
    fn from(s: &str) -> Self {
        Ident(Arc::from(s))
    }
}

impl From<String> for Ident {
    fn from(s: String) -> Self {
        Ident(Arc::from(s))
    }
}

impl From<&Ident> for Ident {
    fn from(s: &Ident) -> Self {
        s.clone()
    }
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub fn intern(s: &str) -> Ident {
    s.into()
}

pub fn intern_string(s: String) -> Ident {
    s.into()
}

pub fn display_ident(ident: &Ident, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(ident.as_ref())
}

pub trait IdentDisplay {
    fn fmt_ident(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl IdentDisplay for Ident {
    fn fmt_ident(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_ident(self, f)
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

#[cfg(test)]
mod _ident_safety_check {
    use super::*;
    const _IS_IDENT: () = {
        fn assert_ident(_: &Ident) {}
        fn _check() {
            use std::any::TypeId;
            assert_ne!(TypeId::of::<Ident>(), TypeId::of::<String>());
        }
    };
}
