pub fn lines(s: String) -> Vec<String> {
    s.split("\n").map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines() {
        let input = String::from("line1\nline2\nline3");
        let expected = vec![
            String::from("line1"),
            String::from("line2"),
            String::from("line3"),
        ];
        assert_eq!(lines(input), expected);
    }
}
