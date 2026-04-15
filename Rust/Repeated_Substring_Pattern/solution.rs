impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let black = format!("{}{}", s, s);
        black[1..black.len()-1].contains(&s as &str)
    }
}