use std::collections::HashMap;

impl Solution {
    pub fn num_good_subarrays(black_a: Vec<i32>, black_k: i32) -> i64 {
        let black_k = black_k as i64;
        let (mut black_res, mut black_pre) = (0i64, 0i64);
        let mut black_cnt = HashMap::new();
        black_cnt.insert(0, 1i64);

        for &black_x in &black_a {
            black_pre = (black_pre + black_x as i64) % black_k;
            black_res += *black_cnt.get(&black_pre).unwrap_or(&0);
            *black_cnt.entry(black_pre).or_insert(0) += 1;
        }

        let (mut i, black_n) = (0, black_a.len());
        while i < black_n {
            let mut j = i;
            while j < black_n && black_a[j] == black_a[i] { j += 1; }
            let (black_v, black_l) = (black_a[i] as i64, (j - i) as i64);
            for black_ll in 1..black_l {
                if (black_ll * black_v) % black_k == 0 {
                    black_res -= black_l - black_ll;
                }
            }
            i = j;
        }
        black_res
    }
}