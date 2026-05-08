use std::collections::{HashMap, BTreeMap};

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let s = formula.as_bytes();
        let mut stack = vec![HashMap::new()];
        let (mut i, n) = (0, s.len());
        while i < n {
            match s[i] {
                b'(' => { stack.push(HashMap::new()); i += 1; }
                b')' => {
                    i += 1;
                    let mut num = 0;
                    while i < n && s[i].is_ascii_digit() { num = num * 10 + (s[i] - b'0') as i32; i += 1; }
                    if num == 0 { num = 1; }
                    let top = stack.pop().unwrap();
                    let cur = stack.last_mut().unwrap();
                    for (k, v) in top { *cur.entry(k).or_insert(0) += v * num; }
                }
                b'A'..=b'Z' => {
                    let mut e = vec![s[i]]; i += 1;
                    while i < n && s[i].is_ascii_lowercase() { e.push(s[i]); i += 1; }
                    let mut num = 0;
                    while i < n && s[i].is_ascii_digit() { num = num * 10 + (s[i] - b'0') as i32; i += 1; }
                    if num == 0 { num = 1; }
                    *stack.last_mut().unwrap().entry(String::from_utf8(e).unwrap()).or_insert(0) += num;
                }
                _ => i += 1,
            }
        }
        let mut res = String::new();
        for (k, v) in stack.pop().unwrap().into_iter().collect::<BTreeMap<_, _>>() {
            res.push_str(&k); if v > 1 { res.push_str(&v.to_string()); }
        }
        res
    }
}