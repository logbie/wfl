use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

static INTERN_POOL: Lazy<Mutex<HashSet<Arc<str>>>> = Lazy::new(|| {
    Mutex::new(HashSet::new())
});

pub fn intern(s: &str) -> Arc<str> {
    let pool = INTERN_POOL.lock().unwrap();
    if let Some(interned) = pool.get(s) {
        return Arc::clone(interned);
    }
    drop(pool); // Release the lock before modifying
    
    let arc_str = Arc::from(s);
    let mut pool = INTERN_POOL.lock().unwrap();
    if let Some(interned) = pool.get(s) {
        return Arc::clone(interned);
    }
    pool.insert(Arc::clone(&arc_str));
    arc_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intern_same_string() {
        let s1 = intern("hello");
        let s2 = intern("hello");
        
        assert!(Arc::ptr_eq(&s1, &s2));
    }

    #[test]
    fn test_intern_different_strings() {
        let s1 = intern("hello");
        let s2 = intern("world");
        
        assert!(!Arc::ptr_eq(&s1, &s2));
    }
}
