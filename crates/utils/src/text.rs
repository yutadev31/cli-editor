pub fn lines(s: String) -> Vec<String> {
    s.split("\n").map(|s| s.to_string()).collect()
}
