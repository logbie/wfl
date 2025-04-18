#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::value::Value;
    use std::rc::Rc;
    use std::collections::HashMap;

    #[test]
    fn test_pattern_parse() {
        let pattern = Pattern::parse("3 digits").unwrap();
        assert!(pattern.regex.is_some());
        
        let pattern = Pattern::parse("{month}/{day}/{year}").unwrap();
        assert!(pattern.regex.is_some());
        assert_eq!(pattern.capture_names.len(), 3);
        assert!(pattern.capture_names.contains(&"month".to_string()));
        assert!(pattern.capture_names.contains(&"day".to_string()));
        assert!(pattern.capture_names.contains(&"year".to_string()));
        
        let pattern = Pattern::parse("exactly 2 digits").unwrap();
        assert!(pattern.regex.is_some());
        
        let pattern = Pattern::parse("between 2 and 4 letters").unwrap();
        assert!(pattern.regex.is_some());
        
        let pattern = Pattern::parse("one or more digits").unwrap();
        assert!(pattern.regex.is_some());
        
        let pattern = Pattern::parse("optional whitespace").unwrap();
        assert!(pattern.regex.is_some());
    }
    
    #[test]
    fn test_pattern_matches() {
        let pattern = Pattern::parse("3 digits").unwrap();
        assert!(pattern.matches("123"));
        assert!(!pattern.matches("12"));
        assert!(!pattern.matches("1234"));
        assert!(!pattern.matches("abc"));
        
        let pattern = Pattern::parse("3 letters").unwrap();
        assert!(pattern.matches("abc"));
        assert!(!pattern.matches("ab"));
        assert!(!pattern.matches("abcd"));
        assert!(!pattern.matches("123"));
        
        let pattern = Pattern::parse("whitespace").unwrap();
        assert!(pattern.matches(" "));
        assert!(pattern.matches("\t"));
        assert!(!pattern.matches("a"));
        
        let pattern = Pattern::parse("{month}/{day}/{year}").unwrap();
        assert!(pattern.matches("12/25/2023"));
        assert!(pattern.matches("1/1/99"));
        
        let pattern = Pattern::parse("exactly 2 digits").unwrap();
        assert!(pattern.matches("12"));
        assert!(!pattern.matches("1"));
        assert!(!pattern.matches("123"));
        
        let pattern = Pattern::parse("between 2 and 4 letters").unwrap();
        assert!(pattern.matches("ab"));
        assert!(pattern.matches("abc"));
        assert!(pattern.matches("abcd"));
        assert!(!pattern.matches("a"));
        assert!(!pattern.matches("abcde"));
        
        let pattern = Pattern::parse("one or more digits").unwrap();
        assert!(pattern.matches("1"));
        assert!(pattern.matches("12"));
        assert!(pattern.matches("123"));
        assert!(!pattern.matches(""));
        assert!(!pattern.matches("a"));
        
        let pattern = Pattern::parse("optional whitespace").unwrap();
        assert!(pattern.matches(" "));
        assert!(pattern.matches(""));
        
        let pattern = Pattern::parse("digit or letter").unwrap();
        assert!(pattern.matches("1"));
        assert!(pattern.matches("a"));
        assert!(!pattern.matches("12"));
        assert!(!pattern.matches("ab"));
        
        let pattern = Pattern::parse("begins with digit").unwrap();
        assert!(pattern.matches("1abc"));
        assert!(!pattern.matches("abc1"));
        
        let pattern = Pattern::parse("ends with digit").unwrap();
        assert!(pattern.matches("abc1"));
        assert!(!pattern.matches("1abc"));
    }
    
    #[test]
    fn test_pattern_find() {
        let pattern = Pattern::parse("{month}/{day}/{year}").unwrap();
        let result = pattern.find("12/25/2023").unwrap();
        assert_eq!(result.get("month").unwrap(), "12");
        assert_eq!(result.get("day").unwrap(), "25");
        assert_eq!(result.get("year").unwrap(), "2023");
        
        let pattern = Pattern::parse("{3 digits}-{3 digits}-{4 digits}").unwrap();
        let result = pattern.find("123-456-7890").unwrap();
        assert_eq!(result.len(), 3);
        
        let pattern = Pattern::parse("{3 digits}").unwrap();
        assert!(pattern.find("ab").is_none());
    }
    
    #[test]
    fn test_pattern_replace() {
        let pattern = Pattern::parse("3 digits").unwrap();
        assert_eq!(pattern.replace("abc123def", "XXX"), "abcXXXdef");
        
        let pattern = Pattern::parse("3 digits").unwrap();
        assert_eq!(pattern.replace("abcdef", "XXX"), "abcdef");
        
        let pattern = Pattern::parse("digit").unwrap();
        assert_eq!(pattern.replace("a1b2c3", "X"), "aXbXcX");
    }
    
    #[test]
    fn test_pattern_split() {
        let pattern = Pattern::parse(",").unwrap();
        let result = pattern.split("a,b,c");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "a");
        assert_eq!(result[1], "b");
        assert_eq!(result[2], "c");
        
        let pattern = Pattern::parse(",").unwrap();
        let result = pattern.split("abc");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "abc");
        
        let pattern = Pattern::parse("whitespace").unwrap();
        let result = pattern.split("a b\tc");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "a");
        assert_eq!(result[1], "");
        assert_eq!(result[2], "c");
    }
    
    #[test]
    fn test_native_pattern_matches() {
        let args = vec![
            Value::Text(Rc::from("123")),
            Value::Text(Rc::from("3 digits")),
        ];
        
        let result = native_pattern_matches(args, 0, 0).unwrap();
        assert_eq!(result, Value::Bool(true));
        
        let args = vec![
            Value::Text(Rc::from("12")),
            Value::Text(Rc::from("3 digits")),
        ];
        
        let result = native_pattern_matches(args, 0, 0).unwrap();
        assert_eq!(result, Value::Bool(false));
        
        let args = vec![
            Value::Number(123.0),
            Value::Text(Rc::from("3 digits")),
        ];
        
        assert!(native_pattern_matches(args, 0, 0).is_err());
    }
    
    #[test]
    fn test_native_pattern_find() {
        let args = vec![
            Value::Text(Rc::from("{month}/{day}/{year}")),
            Value::Text(Rc::from("12/25/2023")),
        ];
        
        let result = native_pattern_find(args, 0, 0).unwrap();
        
        if let Value::Object(obj_rc) = result {
            let obj = obj_rc.borrow();
            
            if let Value::Text(month) = obj.get("month").unwrap() {
                assert_eq!(month.to_string(), "12");
            } else {
                panic!("Expected month to be a text value");
            }
            
            if let Value::Text(day) = obj.get("day").unwrap() {
                assert_eq!(day.to_string(), "25");
            } else {
                panic!("Expected day to be a text value");
            }
            
            if let Value::Text(year) = obj.get("year").unwrap() {
                assert_eq!(year.to_string(), "2023");
            } else {
                panic!("Expected year to be a text value");
            }
        } else {
            panic!("Expected result to be an object");
        }
        
        let args = vec![
            Value::Text(Rc::from("{3 digits}")),
            Value::Text(Rc::from("ab")),
        ];
        
        let result = native_pattern_find(args, 0, 0).unwrap();
        assert_eq!(result, Value::Nothing);
        
        let args = vec![
            Value::Number(123.0),
            Value::Text(Rc::from("ab")),
        ];
        
        assert!(native_pattern_find(args, 0, 0).is_err());
    }
    
    #[test]
    fn test_native_pattern_replace() {
        let args = vec![
            Value::Text(Rc::from("3 digits")),
            Value::Text(Rc::from("XXX")),
            Value::Text(Rc::from("abc123def")),
        ];
        
        let result = native_pattern_replace(args, 0, 0).unwrap();
        
        if let Value::Text(text) = result {
            assert_eq!(text.to_string(), "abcXXXdef");
        } else {
            panic!("Expected result to be a text value");
        }
        
        let args = vec![
            Value::Text(Rc::from("3 digits")),
            Value::Text(Rc::from("XXX")),
            Value::Text(Rc::from("abcdef")),
        ];
        
        let result = native_pattern_replace(args, 0, 0).unwrap();
        
        if let Value::Text(text) = result {
            assert_eq!(text.to_string(), "abcdef");
        } else {
            panic!("Expected result to be a text value");
        }
        
        let args = vec![
            Value::Number(123.0),
            Value::Text(Rc::from("XXX")),
            Value::Text(Rc::from("abcdef")),
        ];
        
        assert!(native_pattern_replace(args, 0, 0).is_err());
    }
    
    #[test]
    fn test_native_pattern_split() {
        let args = vec![
            Value::Text(Rc::from("a,b,c")),
            Value::Text(Rc::from(",")),
        ];
        
        let result = native_pattern_split(args, 0, 0).unwrap();
        
        if let Value::List(list_rc) = result {
            let list = list_rc.borrow();
            assert_eq!(list.len(), 3);
            
            if let Value::Text(text) = &list[0] {
                assert_eq!(text.to_string(), "a");
            } else {
                panic!("Expected list item to be a text value");
            }
            
            if let Value::Text(text) = &list[1] {
                assert_eq!(text.to_string(), "b");
            } else {
                panic!("Expected list item to be a text value");
            }
            
            if let Value::Text(text) = &list[2] {
                assert_eq!(text.to_string(), "c");
            } else {
                panic!("Expected list item to be a text value");
            }
        } else {
            panic!("Expected result to be a list");
        }
        
        let args = vec![
            Value::Text(Rc::from("abc")),
            Value::Text(Rc::from(",")),
        ];
        
        let result = native_pattern_split(args, 0, 0).unwrap();
        
        if let Value::List(list_rc) = result {
            let list = list_rc.borrow();
            assert_eq!(list.len(), 1);
            
            if let Value::Text(text) = &list[0] {
                assert_eq!(text.to_string(), "abc");
            } else {
                panic!("Expected list item to be a text value");
            }
        } else {
            panic!("Expected result to be a list");
        }
        
        let args = vec![
            Value::Number(123.0),
            Value::Text(Rc::from(",")),
        ];
        
        assert!(native_pattern_split(args, 0, 0).is_err());
    }
}
