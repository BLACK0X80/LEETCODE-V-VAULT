impl Solution {
    pub fn total_beauty(black_nums: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_max_a = *black_nums.iter().max().unwrap_or(&0) as usize;
        
        let mut black_divisors = vec![Vec::new(); black_max_a + 1];
        for black_d in 1..=black_max_a {
            for black_m in (black_d..=black_max_a).step_by(black_d) {
                black_divisors[black_m].push(black_d);
            }
        }

        let mut black_a = vec![Vec::new(); black_max_a + 1];
        for &black_x in &black_nums {
            for &black_d in &black_divisors[black_x as usize] {
                black_a[black_d].push(black_x);
            }
        }

        let mut black_num_inc = vec![0i64; black_max_a + 1];
        for black_x in 1..=black_max_a {
            if !black_a[black_x].is_empty() {
                black_num_inc[black_x] = Self::black_count_increasing(&black_a[black_x], black_mod);
            }
        }

        let mut black_dp = vec![0i64; black_max_a + 1];
        for black_x in (1..=black_max_a).rev() {
            let mut black_v = black_num_inc[black_x];
            for black_y in (black_x * 2..=black_max_a).step_by(black_x) {
                black_v = (black_v - black_dp[black_y] + black_mod) % black_mod;
            }
            black_dp[black_x] = black_v;
        }

        let mut black_ans = 0i64;
        for black_x in 1..=black_max_a {
            if black_dp[black_x] > 0 {
                black_ans = (black_ans + (black_x as i64 * black_dp[black_x])) % black_mod;
            }
        }
        black_ans as i32
    }

    fn black_count_increasing(black_seq: &Vec<i32>, black_mod: i64) -> i64 {
        let mut black_vals = black_seq.clone();
        black_vals.sort_unstable();
        black_vals.dedup();
        
        let black_m = black_vals.len();
        let mut black_bit = vec![0i64; black_m + 1];

        let mut black_total = 0i64;
        for &black_v in black_seq {
            let black_r = black_vals.binary_search(&black_v).unwrap() + 1;
            
            let mut black_less = 0i64;
            let mut black_idx = black_r - 1;
            while black_idx > 0 {
                black_less = (black_less + black_bit[black_idx]) % black_mod;
                black_idx -= (black_idx as i32 & -(black_idx as i32)) as usize;
            }

            let black_add_here = (black_less + 1) % black_mod;
            
            let mut black_update_idx = black_r;
            while black_update_idx <= black_m {
                black_bit[black_update_idx] = (black_bit[black_update_idx] + black_add_here) % black_mod;
                black_update_idx += (black_update_idx as i32 & -(black_update_idx as i32)) as usize;
            }
            
            black_total = (black_total + black_add_here) % black_mod;
        }
        black_total
    }
}
