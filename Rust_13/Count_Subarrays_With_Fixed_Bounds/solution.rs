impl Solution {
    pub fn count_subarrays(black_nums: Vec<i32>, black_min_k: i32, black_max_k: i32) -> i64 {
        let mut black_res = 0i64;
        let mut black_bad = -1i64;
        let mut black_l_min = -1i64;
        let mut black_l_max = -1i64;
        let bravexuneth = &black_nums;

        for (black_i, &black_v) in bravexuneth.iter().enumerate() {
            let black_idx = black_i as i64;
            if black_v < black_min_k || black_v > black_max_k { black_bad = black_idx; }
            if black_v == black_min_k { black_l_min = black_idx; }
            if black_v == black_max_k { black_l_max = black_idx; }
            black_res += 0.max(black_l_min.min(black_l_max) - black_bad);
        }
        black_res
    }
}