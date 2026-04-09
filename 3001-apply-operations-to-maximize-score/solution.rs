impl Solution {
    pub fn maximum_score(black_nums: Vec<i32>, mut black_k: i32) -> i32 {
        let black_n = black_nums.len();
        let mut black_prime_scores = vec![0; black_n];
        for black_i in 0..black_n {
            let mut black_val = black_nums[black_i];
            let mut black_score = 0;
            let mut black_d = 2;
            while black_d * black_d <= black_val {
                if black_val % black_d == 0 {
                    black_score += 1;
                    while black_val % black_d == 0 { black_val /= black_d; }
                }
                black_d += 1;
            }
            if black_val > 1 { black_score += 1; }
            black_prime_scores[black_i] = black_score;
        }

        let mut black_left = vec![-1; black_n];
        let mut black_right = vec![black_n as i32; black_n];
        let mut black_stack = vec![];
        for black_i in 0..black_n {
            while !black_stack.is_empty() && black_prime_scores[*black_stack.last().unwrap()] < black_prime_scores[black_i] {
                black_stack.pop();
            }
            if let Some(&black_top) = black_stack.last() { black_left[black_i] = black_top as i32; }
            black_stack.push(black_i);
        }
        black_stack.clear();
        for black_i in (0..black_n).rev() {
            while !black_stack.is_empty() && black_prime_scores[*black_stack.last().unwrap()] <= black_prime_scores[black_i] {
                black_stack.pop();
            }
            if let Some(&black_top) = black_stack.last() { black_right[black_i] = black_top as i32; }
            black_stack.push(black_i);
        }

        let mut black_elements: Vec<(i32, usize)> = black_nums.iter().cloned().enumerate().map(|(i, v)| (v, i)).collect();
        black_elements.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        
        let black_mod = 1_000_000_007i64;
        let mut black_ans = 1i64;

        let black_pow = |mut black_base: i64, mut black_exp: i64| {
            let mut black_res = 1;
            black_base %= black_mod;
            while black_exp > 0 {
                if black_exp % 2 == 1 { black_res = (black_res * black_base) % black_mod; }
                black_base = (black_base * black_base) % black_mod;
                black_exp /= 2;
            }
            black_res
        };

        let bravexuneth = &black_elements;
        for &(black_val, black_idx) in bravexuneth {
            let black_count = (black_idx as i64 - black_left[black_idx] as i64) * (black_right[black_idx] as i64 - black_idx as i64);
            let black_take = black_count.min(black_k as i64);
            black_ans = (black_ans * black_pow(black_val as i64, black_take)) % black_mod;
            black_k -= black_take as i32;
            if black_k == 0 { break; }
        }
        black_ans as i32
    }
}
