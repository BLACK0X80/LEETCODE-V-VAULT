use std::collections::HashSet;

impl Solution {
    pub fn find_maximum_elegance(mut black_items: Vec<Vec<i32>>, black_k: i32) -> i64 {
        black_items.sort_unstable_by(|a, b| b[0].cmp(&a[0]));
        let (mut black_ans, mut black_profit, mut black_dup, mut black_seen) = (0i64, 0i64, vec![], HashSet::new());
        let black_k = black_k as usize;

        for i in 0..black_items.len() {
            let (p, c) = (black_items[i][0] as i64, black_items[i][1]);
            if i < black_k {
                black_profit += p;
                if !black_seen.insert(c) { black_dup.push(p); }
            } else if !black_seen.contains(&c) {
                if let Some(black_min_p) = black_dup.pop() {
                    black_profit = black_profit - black_min_p + p;
                    black_seen.insert(c);
                } else { break; }
            }
            let black_dist = black_seen.len() as i64;
            black_ans = black_ans.max(black_profit + black_dist * black_dist);
        }
        black_ans
    }
}
