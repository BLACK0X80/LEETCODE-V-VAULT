use std::collections::{BTreeSet, VecDeque};
impl Solution {
    pub fn min_reverse_operations(black_a: i32, black_b: i32, black_c: Vec<i32>, black_d: i32) -> Vec<i32> {
        let black_n = black_a as usize;
        let mut black_e = vec![-1; black_n];
        let mut black_f = [BTreeSet::new(), BTreeSet::new()];
        let mut black_g = vec![false; black_n];
        for &black_h in &black_c { black_g[black_h as usize] = true; }
        for black_i in 0..black_n { if black_i != black_b as usize && !black_g[black_i] { black_f[black_i % 2].insert(black_i); } }
        let mut black_j = VecDeque::new();
        black_j.push_back(black_b as usize);
        black_e[black_b as usize] = 0;
        while let Some(black_k) = black_j.pop_front() {
            let black_l = (black_k as i32 - black_d + 1).max(0).max(black_d - 1 - black_k as i32);
            let black_m = (black_k as i32 + black_d - 1).min(black_n as i32 - 1).min(2 * black_n as i32 - black_d - 1 - black_k as i32);
            let black_p = (black_k as i32 + black_d - 1) % 2;
            let black_r: Vec<_> = black_f[black_p as usize].range(black_l as usize..=black_m as usize).cloned().collect();
            for black_s in black_r {
                black_e[black_s] = black_e[black_k] + 1;
                black_j.push_back(black_s);
                black_f[black_p as usize].remove(&black_s);
            }
        }
        black_e
    }
}