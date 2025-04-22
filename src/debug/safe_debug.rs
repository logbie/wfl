use std::collections::HashMap;
use std::fmt;

pub trait SafeDebug {
    fn safe_fmt<W: fmt::Write>(&self, f: &mut W) -> fmt::Result;
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

pub fn format_collection<T, F, W>(
    items: &[T],
    f: &mut W,
    format_item: F,
    open: &str,
    close: &str,
) -> fmt::Result
where
    W: fmt::Write,
    F: Fn(&T, &mut W) -> fmt::Result,
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

pub fn format_map<K, V, FK, FV, W>(
    map: &HashMap<K, V>,
    f: &mut W,
    format_key: FK,
    format_value: FV,
) -> fmt::Result
where
    W: fmt::Write,
    FK: Fn(&K, &mut W) -> fmt::Result,
    FV: Fn(&V, &mut W) -> fmt::Result,
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
    use std::fmt::Write;

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

    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    impl TestFormatter {
        fn new() -> Self {
            TestFormatter {
                output: String::new(),
            }
        }
    }

    #[test]
    fn test_format_collection() {
        let items = vec![1, 2, 3, 4, 5];
        let mut test_fmt = TestFormatter::new();

        let result = format_collection(
            &items,
            &mut test_fmt,
            |item, f| write!(f, "{}", item),
            "[",
            "]",
        );

        assert!(result.is_ok());
        assert_eq!(test_fmt.output, "[1, 2, 3, 4, 5]");

        let items: Vec<i32> = (1..=20).collect();
        let mut test_fmt = TestFormatter::new();

        let result = format_collection(
            &items,
            &mut test_fmt,
            |item, f| write!(f, "{}", item),
            "[",
            "]",
        );

        assert!(result.is_ok());
        assert!(test_fmt.output.contains("… (4 more)"));
    }
}
