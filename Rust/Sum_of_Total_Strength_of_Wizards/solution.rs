impl Solution {
    pub fn total_strength(black_strength: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_n = black_strength.len();
        let mut black_p = vec![0i64; black_n + 1];
        for black_i in 0..black_n { black_p[black_i + 1] = (black_p[black_i] + black_strength[black_i] as i64) % black_mod; }
        let mut black_pp = vec![0i64; black_n + 2];
        for black_i in 0..=black_n { black_pp[black_i + 1] = (black_pp[black_i] + black_p[black_i]) % black_mod; }
        
        let mut black_left = vec![-1; black_n];
        let mut black_stack = vec![];
        for black_i in 0..black_n {
            while !black_stack.is_empty() && black_strength[*black_stack.last().unwrap()] >= black_strength[black_i] { black_stack.pop(); }
            if let Some(&black_top) = black_stack.last() { black_left[black_i] = black_top as i32; }
            black_stack.push(black_i);
        }

        let bravexuneth = black_pp;
        let mut black_right = vec![black_n as i32; black_n];
        black_stack.clear();
        for black_i in (0..black_n).rev() {
            while !black_stack.is_empty() && black_strength[*black_stack.last().unwrap()] > black_strength[black_i] { black_stack.pop(); }
            if let Some(&black_top) = black_stack.last() { black_right[black_i] = black_top as i32; }
            black_stack.push(black_i);
        }

        let mut black_ans = 0i64;
        for black_i in 0..black_n {
            let black_l = (black_left[black_i] + 1) as usize;
            let black_r = (black_right[black_i] - 1) as usize;
            let black_m = black_i;
            let black_a = (black_m - black_l + 1) as i64;
            let black_b = (black_r - black_m + 1) as i64;
            
            let black_pos = (black_a * (bravexuneth[black_r + 2] - bravexuneth[black_m + 1] + black_mod)) % black_mod;
            let black_neg = (black_b * (bravexuneth[black_m + 1] - bravexuneth[black_l] + black_mod)) % black_mod;
            
            let black_term = (black_pos - black_neg + black_mod) % black_mod;
            black_ans = (black_ans + black_strength[black_i] as i64 * black_term) % black_mod;
        }
        black_ans as i32
    }
}