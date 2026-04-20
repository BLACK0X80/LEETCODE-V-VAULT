use std::collections::HashMap;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut memo: HashMap<usize, Vec<String>> = HashMap::new();
        Self::dfs(s.as_bytes(), 0, &word_dict, &mut memo)
    }

    fn dfs(s: &[u8], start: usize, dict: &[String], memo: &mut HashMap<usize, Vec<String>>) -> Vec<String> {
        if let Some(cached) = memo.get(&start) { return cached.clone(); }
        if start == s.len() { return vec![String::new()]; }

        let mut result = vec![];
        for word in dict {
            let end = start + word.len();
            if end <= s.len() && &s[start..end] == word.as_bytes() {
                for rest in Self::dfs(s, end, dict, memo) {
                    result.push(if rest.is_empty() {
                        word.clone()
                    } else {
                        format!("{} {}", word, rest)
                    });
                }
            }
        }

        memo.insert(start, result.clone());
        result
    }
}