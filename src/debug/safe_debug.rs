use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

pub trait SafeDebug {
    fn safe_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

pub fn truncate_utf8_safe(s: &str, max_chars: usize) -> &str {
    if s.chars().count() <= max_chars {
        return s;
    }
    
    match s.char_indices().nth(max_chars) {
        Some((idx, _)) => &s[..idx],
        None => s, // This should never happen given the check above
    }
}

pub fn format_collection<T, F>(
    items: &[T],
    f: &mut fmt::Formatter<'_>,
    format_item: F,
    open: &str,
    close: &str,
) -> fmt::Result
where
    F: Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result,
{
    const MAX_ELEMENTS: usize = 16;
    
    write!(f, "{}", open)?;
    
    let display_count = std::cmp::min(items.len(), MAX_ELEMENTS);
    let remaining = items.len().saturating_sub(MAX_ELEMENTS);
    
    for (i, item) in items.iter().take(display_count).enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        format_item(item, f)?;
    }
    
    if remaining > 0 {
        write!(f, " … ({} more)", remaining)?;
    }
    
    write!(f, "{}", close)
}

pub fn format_map<K, V, FK, FV>(
    map: &HashMap<K, V>,
    f: &mut fmt::Formatter<'_>,
    format_key: FK,
    format_value: FV,
) -> fmt::Result
where
    FK: Fn(&K, &mut fmt::Formatter<'_>) -> fmt::Result,
    FV: Fn(&V, &mut fmt::Formatter<'_>) -> fmt::Result,
{
    const MAX_ELEMENTS: usize = 16;
    
    write!(f, "{{")?;
    
    let entries: Vec<_> = map.iter().collect();
    let display_count = std::cmp::min(entries.len(), MAX_ELEMENTS);
    let remaining = entries.len().saturating_sub(MAX_ELEMENTS);
    
    for (i, (key, value)) in entries.iter().take(display_count).enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }
        format_key(key, f)?;
        write!(f, ": ")?;
        format_value(value, f)?;
    }
    
    if remaining > 0 {
        write!(f, " … ({} more)", remaining)?;
    }
    
    write!(f, "}}")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_truncate_utf8_safe() {
        let s = "Hello, world!";
        assert_eq!(truncate_utf8_safe(s, 5), "Hello");
        
        let s = "こんにちは世界"; // "Hello world" in Japanese
        assert_eq!(truncate_utf8_safe(s, 3), "こんに");
        
        let s = "abc";
        assert_eq!(truncate_utf8_safe(s, 5), "abc");
        
        let s = "";
        assert_eq!(truncate_utf8_safe(s, 5), "");
    }
    
    #[test]
    fn test_format_collection() {
        let items = vec![1, 2, 3, 4, 5];
        let mut output = String::new();
        let mut formatter = fmt::Formatter::new(&mut output);
        
        format_collection(&items, &mut formatter, |item, f| write!(f, "{}", item), "[", "]").unwrap();
        
        assert_eq!(output, "[1, 2, 3, 4, 5]");
        
        let items: Vec<i32> = (1..=20).collect();
        let mut output = String::new();
        let mut formatter = fmt::Formatter::new(&mut output);
        
        format_collection(&items, &mut formatter, |item, f| write!(f, "{}", item), "[", "]").unwrap();
        
        assert!(output.contains("… (4 more)"));
    }
}
