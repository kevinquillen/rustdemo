use regex::Regex;

pub fn find_match(content: String, pattern: &str) -> bool {
    return Regex::new(format!(r#"(?i){}"#, pattern).as_str()).unwrap().is_match(&content.as_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_match() {
        let content = String::from("Hello there.");
        let pattern = "there";
        assert_eq!(true, find_match(content, pattern));

        let content = String::from("Goodbye!");
        let pattern = "GOOD";
        assert_eq!(true, find_match(content, pattern));

        let content = String::from("I am a ParTiAL mixed case");
        let pattern = "part";
        assert_eq!(true, find_match(content, pattern));

        let content = String::from("good bye");
        let pattern = "hello";
        assert_eq!(false, find_match(content, pattern));

        let content = String::from("hel lo");
        let pattern = "hello";
        assert_eq!(false, find_match(content, pattern));
    }
}