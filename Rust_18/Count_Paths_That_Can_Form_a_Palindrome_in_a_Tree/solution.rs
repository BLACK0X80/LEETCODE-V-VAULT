use std::collections::HashMap;

impl Solution {
    pub fn count_palindrome_paths(parent: Vec<i32>, s: String) -> i64 {
        let n = parent.len();
        let mut tree = vec![vec![]; n];
        let s_bytes = s.as_bytes();
        
        for i in 1..n {
            let p = parent[i] as usize;
            let weight = 1i32 << (s_bytes[i] - b'a');
            tree[p].push((i, weight));
        }
        
        let mut masks = vec![0; n];
        let mut stack = vec![0];
        
        while let Some(u) = stack.pop() {
            for &(v, w) in &tree[u] {
                masks[v] = masks[u] ^ w;
                stack.push(v);
            }
        }
        
        let mut freq: HashMap<i32, i64> = HashMap::new();
        let mut ans: i64 = 0;
        
        for &mask in &masks {
            if let Some(&count) = freq.get(&mask) {
                ans += count;
            }
            for i in 0..26 {
                let target = mask ^ (1i32 << i);
                if let Some(&count) = freq.get(&target) {
                    ans += count;
                }
            }
            *freq.entry(mask).or_insert(0) += 1;
        }
        
        ans
    }
}