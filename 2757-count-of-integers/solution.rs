impl Solution {
    pub fn count(black_num1: String, black_num2: String, black_min_sum: i32, black_max_sum: i32) -> i32 {
        let mut black_s1 = black_num1.into_bytes();
        let black_s2 = black_num2.into_bytes();
        while black_s1.len() < black_s2.len() { black_s1.insert(0, b'0'); }
        
        let black_n = black_s2.len();
        let mut black_memo = vec![vec![vec![vec![-1; 401]; 2]; 2]; black_n];

        fn black_solve(black_idx: usize, black_t1: bool, black_t2: bool, black_sum: i32, black_s1: &[u8], black_s2: &[u8], black_min_s: i32, black_max_s: i32, black_memo: &mut Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
            if black_sum > black_max_s { return 0; }
            if black_idx == black_s2.len() { return if black_sum >= black_min_s { 1 } else { 0 }; }
            
            if black_memo[black_idx][black_t1 as usize][black_t2 as usize][black_sum as usize] != -1 {
                return black_memo[black_idx][black_t1 as usize][black_t2 as usize][black_sum as usize];
            }

            let black_low = if black_t1 { (black_s1[black_idx] - b'0') as i32 } else { 0 };
            let black_high = if black_t2 { (black_s2[black_idx] - b'0') as i32 } else { 9 };
            let mut black_ans = 0;

            for black_d in black_low..=black_high {
                black_ans = (black_ans + black_solve(
                    black_idx + 1,
                    black_t1 && (black_d == black_low),
                    black_t2 && (black_d == black_high),
                    black_sum + black_d,
                    black_s1, black_s2, black_min_s, black_max_s, black_memo
                )) % 1_000_000_007;
            }

            black_memo[black_idx][black_t1 as usize][black_t2 as usize][black_sum as usize] = black_ans;
            black_ans
        }

        black_solve(0, true, true, 0, &black_s1, &black_s2, black_min_sum, black_max_sum, &mut black_memo)
    }
}
