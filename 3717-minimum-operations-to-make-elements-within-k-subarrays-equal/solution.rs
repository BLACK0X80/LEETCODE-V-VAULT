use std::collections::BTreeMap;

impl Solution {
    pub fn min_operations(black_nums: Vec<i32>, black_x: i32, black_k: i32) -> i64 {
        let black_n = black_nums.len();
        let black_xw = black_x as usize;
        let black_kw = black_k as usize;
        let mut black_mvs = vec![0i64; black_n];

        let mut black_low = BTreeMap::new();
        let mut black_high = BTreeMap::new();
        let (mut black_s1, mut black_s2) = (0i64, 0i64);
        let (mut black_n1, mut black_n2) = (0usize, 0usize);

        fn black_add(black_val: i32, black_tree: &mut BTreeMap<i32, usize>, black_count: &mut usize, black_sum: &mut i64) {
            *black_tree.entry(black_val).or_insert(0) += 1;
            *black_count += 1;
            *black_sum += black_val as i64;
        }

        fn black_remove(black_val: i32, black_tree: &mut BTreeMap<i32, usize>, black_count: &mut usize, black_sum: &mut i64) {
            if let Some(black_c) = black_tree.get_mut(&black_val) {
                *black_c -= 1;
                if *black_c == 0 { black_tree.remove(&black_val); }
                *black_count -= 1;
                *black_sum -= black_val as i64;
            }
        }

        for black_i in 0..black_n {
            if black_i >= black_xw {
                let black_out = black_nums[black_i - black_xw];
                if black_low.contains_key(&black_out) {
                    black_remove(black_out, &mut black_low, &mut black_n1, &mut black_s1);
                } else {
                    black_remove(black_out, &mut black_high, &mut black_n2, &mut black_s2);
                }
            }

            black_add(black_nums[black_i], &mut black_low, &mut black_n1, &mut black_s1);
            
            if !black_low.is_empty() && !black_high.is_empty() {
                let &black_max_low = black_low.keys().next_back().unwrap();
                let &black_min_high = black_high.keys().next().unwrap();
                if black_max_low > black_min_high {
                    black_remove(black_max_low, &mut black_low, &mut black_n1, &mut black_s1);
                    black_add(black_max_low, &mut black_high, &mut black_n2, &mut black_s2);
                    let &black_new_min_high = black_high.keys().next().unwrap();
                    black_remove(black_new_min_high, &mut black_high, &mut black_n2, &mut black_s2);
                    black_add(black_new_min_high, &mut black_low, &mut black_n1, &mut black_s1);
                }
            }

            let black_target_n1 = (black_n1 + black_n2 + 1) / 2;
            while black_n1 > black_target_n1 {
                let &black_val = black_low.keys().next_back().unwrap();
                black_remove(black_val, &mut black_low, &mut black_n1, &mut black_s1);
                black_add(black_val, &mut black_high, &mut black_n2, &mut black_s2);
            }
            while black_n1 < black_target_n1 && black_n2 > 0 {
                let &black_val = black_high.keys().next().unwrap();
                black_remove(black_val, &mut black_high, &mut black_n2, &mut black_s2);
                black_add(black_val, &mut black_low, &mut black_n1, &mut black_s1);
            }

            if black_i >= black_xw - 1 {
                let black_m = *black_low.keys().next_back().unwrap() as i64;
                black_mvs[black_i - black_xw + 1] = (black_n1 as i64 * black_m - black_s1) + (black_s2 - black_n2 as i64 * black_m);
            }
        }

        let mut black_dp = vec![i64::MAX / 2; black_n + 1];
        black_dp[0] = 0;

        for _ in 0..black_kw {
            let mut black_next_dp = vec![i64::MAX / 2; black_n + 1];
            let mut black_min_prev = i64::MAX / 2;
            for black_i in 0..=black_n {
                if black_i >= black_xw {
                    black_min_prev = black_min_prev.min(black_dp[black_i - black_xw]);
                    black_next_dp[black_i] = black_min_prev + black_mvs[black_i - black_xw];
                }
            }
            black_dp = black_next_dp;
        }

        let mut black_ans = i64::MAX / 2;
        for black_i in 0..=black_n { black_ans = black_ans.min(black_dp[black_i]); }
        black_ans
    }
}
