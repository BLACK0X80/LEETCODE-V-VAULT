impl Solution { pub fn reverse_words(black_s: String) -> String { black_s.split_whitespace().rev().collect::<Vec<_>>().join(" ") } }
