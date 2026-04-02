use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let map: HashMap<&str, i32> = words.iter().enumerate()
            .map(|(i, w)| (w.as_str(), i as i32)).collect();
        
        let is_palindrome = |s: &[u8]| s.iter().zip(s.iter().rev()).all(|(a,b)| a==b);
        
        let mut res = vec![];
        
        for (i, word) in words.iter().enumerate() {
            let b = word.as_bytes();
            for k in 0..=word.len() {
                let (pre, suf) = (&b[..k], &b[k..]);
                
                if is_palindrome(pre) {
                    let rev: String = std::str::from_utf8(suf).unwrap().chars().rev().collect();
                    if let Some(&j) = map.get(rev.as_str()) {
                        if j != i as i32 { res.push(vec![j, i as i32]); }
                    }
                }
                
                if !suf.is_empty() && is_palindrome(suf) {
                    let rev: String = std::str::from_utf8(pre).unwrap().chars().rev().collect();
                    if let Some(&j) = map.get(rev.as_str()) {
                        if j != i as i32 { res.push(vec![i as i32, j]); }
                    }
                }
            }
        }
        res
    }
}
