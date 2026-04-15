use std::collections::HashMap;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
        let mut children: Vec<HashMap<(u8,u8), usize>> = vec![HashMap::new()];
        let mut cnt: Vec<i64> = vec![0];
        let mut ans = 0i64;

        for word in &words {
            let w = word.as_bytes();
            let m = w.len();
            let mut node = 0usize;
            ans += cnt[node];
            for i in 0..m {
                let key = (w[i], w[m-1-i]);
                let next = if let Some(&n) = children[node].get(&key) {
                    n
                } else {
                    let n = children.len();
                    children.push(HashMap::new());
                    cnt.push(0);
                    children[node].insert(key, n);
                    n
                };
                node = next;
                ans += cnt[node];
            }
            cnt[node] += 1;
        }
        ans
    }
}