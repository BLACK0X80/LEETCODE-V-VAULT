use std::collections::HashMap;

struct WordFilter {
    map: HashMap<(String, String), i32>,
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut m = HashMap::new();
        for (i, w) in words.iter().enumerate() {
            for a in 0..=w.len() {
                for b in 0..=w.len() {
                    m.insert((w[..a].into(), w[w.len()-b..].into()), i as i32);
                }
            }
        }
        Self { map: m }
    }
    
    fn f(&self, pref: String, suff: String) -> i32 {
        self.map.get(&(pref, suff)).copied().unwrap_or(-1)
    }
}