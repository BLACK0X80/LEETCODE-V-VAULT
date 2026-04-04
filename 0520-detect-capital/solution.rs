impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let b = word.as_bytes();
        b.iter().all(|c| c.is_ascii_uppercase()) ||
        b.iter().all(|c| c.is_ascii_lowercase()) ||
        (b[0].is_ascii_uppercase() && b[1..].iter().all(|c| c.is_ascii_lowercase()))
    }
}
