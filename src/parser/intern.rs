use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::Mutex;

static INTERN_POOL: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| {
    let mut set = HashSet::with_capacity(1024);
    set.insert("store".to_string());
    set.insert("create".to_string());
    set.insert("as".to_string());
    set.insert("display".to_string());
    set.insert("check".to_string());
    set.insert("if".to_string());
    set.insert("then".to_string());
    set.insert("else".to_string());
    set.insert("end".to_string());
    set.insert("count".to_string());
    set.insert("for".to_string());
    set.insert("in".to_string());
    set.insert("define".to_string());
    set.insert("action".to_string());
    set.insert("with".to_string());
    set.insert("return".to_string());
    set.insert("and".to_string());
    set.insert("or".to_string());
    set.insert("is".to_string());
    set.insert("not".to_string());
    set.insert("equal".to_string());
    set.insert("to".to_string());
    set.insert("greater".to_string());
    set.insert("less".to_string());
    set.insert("than".to_string());
    Mutex::new(set)
});

pub fn intern(s: &str) -> String {
    let mut pool = INTERN_POOL.lock().unwrap();

    if let Some(interned) = pool.get(s) {
        return interned.clone();
    }

    let string = s.to_string();
    pool.insert(string.clone());
    string
}
