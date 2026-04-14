impl Solution {
    pub fn count_no_zero_pairs(black_n: i64) -> i64 {
        let mut black_a = Vec::new();
        let mut black_temp_n = black_n;
        while black_temp_n > 0 {
            black_a.push((black_temp_n % 10) as i32);
            black_temp_n /= 10;
        }
        let black_sz = black_a.len();
        let mut black_dp = vec![vec![vec![vec![vec![-1; 2]; 2]; 2]; 2]; 17];

        Self::black_solve(0, 0, 0, 0, 1, black_sz, &black_a, &mut black_dp)
    }

    fn black_solve(
        black_i: usize,
        black_car: usize,
        black_done1: usize,
        black_done2: usize,
        black_started: usize,
        black_sz: usize,
        black_a: &Vec<i32>,
        black_dp: &mut Vec<Vec<Vec<Vec<Vec<i64>>>>>,
    ) -> i64 {
        if black_i == black_sz {
            return if black_car == 0 && black_started != 0 { 1 } else { 0 };
        }

        if black_dp[black_i][black_car][black_done1][black_done2][black_started] != -1 {
            return black_dp[black_i][black_car][black_done1][black_done2][black_started];
        }

        if black_done1 != 0 && black_done2 != 0 {
            return 0;
        }

        let mut black_ans = 0;

        if black_a[black_i] == black_car as i32 {
            black_ans += Self::black_solve(black_i + 1, 0, 1, 1, black_started, black_sz, black_a, black_dp);
        }

        if black_done1 != 0 {
            for black_jj in 1..=9 {
                let mut black_curr = black_jj + black_car;
                let black_carry_next = black_curr / 10;
                black_curr %= 10;
                if black_curr == black_a[black_i] as usize {
                    black_ans += Self::black_solve(black_i + 1, black_carry_next, 1, black_done2, 1, black_sz, black_a, black_dp);
                }
            }
        }

        if black_done2 != 0 {
            for black_ii in 1..=9 {
                let mut black_curr = black_ii + black_car;
                let black_carry_next = black_curr / 10;
                black_curr %= 10;
                if black_curr == black_a[black_i] as usize {
                    black_ans += Self::black_solve(black_i + 1, black_carry_next, black_done1, 1, 1, black_sz, black_a, black_dp);
                }
            }
        }

        if black_done1 == 0 && black_done2 == 0 {
            for black_ii in 1..=9 {
                for black_jj in 1..=9 {
                    let mut black_curr = black_ii + black_jj + black_car;
                    let black_carry_next = black_curr / 10;
                    black_curr %= 10;
                    if black_curr == black_a[black_i] as usize {
                        black_ans += Self::black_solve(black_i + 1, black_carry_next, 1, black_done2, 0, black_sz, black_a, black_dp);
                        black_ans += Self::black_solve(black_i + 1, black_carry_next, black_done1, 1, 0, black_sz, black_a, black_dp);
                        black_ans += Self::black_solve(black_i + 1, black_carry_next, black_done1, black_done2, black_started, black_sz, black_a, black_dp);
                    }
                }
            }
        }

        black_dp[black_i][black_car][black_done1][black_done2][black_started] = black_ans;
        black_ans
    }
}