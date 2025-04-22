use crate::common::ident::Ident;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

static INTERN_POOL: Lazy<Mutex<HashSet<Arc<str>>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub fn intern(s: &str) -> Ident {
    let pool = INTERN_POOL.lock().unwrap();
    if let Some(interned) = pool.get(s) {
        return Ident::from(interned.as_ref());
    }
    drop(pool); // Release the lock before modifying

    let arc_str = Arc::from(s);
    let mut pool = INTERN_POOL.lock().unwrap();
    if let Some(interned) = pool.get(s) {
        return Ident::from(interned.as_ref());
    }
    pool.insert(Arc::clone(&arc_str));
    Ident::from(arc_str.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_intern_same_string() {
        let s1 = intern("hello");
        let s2 = intern("hello");

        assert_eq!(s1.as_str(), s2.as_str());
    }

    #[test]
    fn test_intern_different_strings() {
        let s1 = intern("hello");
        let s2 = intern("world");

        assert_ne!(s1.as_str(), s2.as_str());
    }
}
