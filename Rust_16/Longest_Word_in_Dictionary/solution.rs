use std::collections::HashSet;

impl Solution {
    pub fn longest_word(mut words: Vec<String>) -> String {
        words.sort();
        let mut set = HashSet::new();
        let mut ans = String::new();
        set.insert("".to_string());
        for w in &words {
            let prev = &w[..w.len()-1];
            if set.contains(prev) {
                set.insert(w.clone());
                if w.len() > ans.len() { ans = w.clone(); }
            }
        }
        ans
    }
}