impl Solution {
    pub fn min_travel_time(l: i32, n: i32, k: i32, position: Vec<i32>, time: Vec<i32>) -> i32 {
        let black_n = n as usize;
        let black_k = k as usize;
        let mut black_sum = 0;
        for &black_x in &time {
            black_sum += black_x;
        }

        let mut black_dp = vec![vec![vec![-1; (black_sum + 1) as usize]; black_k + 1]; black_n];
        
        Self::black_recursion(
            0,
            black_k,
            time[0],
            black_n,
            black_k,
            &position,
            &time,
            &mut black_dp,
        )
    }

    fn black_recursion(
        black_pos: usize,
        black_curr_k: usize,
        black_curr_time: i32,
        black_n: usize,
        black_k: usize,
        black_position: &Vec<i32>,
        black_time: &Vec<i32>,
        black_dp: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        if black_pos == black_n - 1 {
            return if black_curr_k > 0 { i32::MAX } else { 0 };
        }

        if black_dp[black_pos][black_curr_k][black_curr_time as usize] != -1 {
            return black_dp[black_pos][black_curr_k][black_curr_time as usize];
        }

        let mut black_ans = i32::MAX;

        let black_res_normal = Self::black_recursion(
            black_pos + 1,
            black_curr_k,
            black_time[black_pos + 1],
            black_n,
            black_k,
            black_position,
            black_time,
            black_dp,
        );

        if black_res_normal != i32::MAX {
            let black_travel = (black_position[black_pos + 1] - black_position[black_pos]) * black_curr_time;
            black_ans = black_ans.min(black_travel + black_res_normal);
        }

        if black_curr_k > 0 {
            let mut black_time_sum = black_time[black_pos + 1];
            let mut black_operations = 0;

            let black_limit = (black_pos + black_curr_k + 1).min(black_n - 1);
            for black_next_idx in (black_pos + 2)..=black_limit {
                black_time_sum += black_time[black_next_idx];
                black_operations += 1;

                let black_res_merge = Self::black_recursion(
                    black_next_idx,
                    black_curr_k - black_operations,
                    black_time_sum,
                    black_n,
                    black_k,
                    black_position,
                    black_time,
                    black_dp,
                );

                if black_res_merge != i32::MAX {
                    let black_travel = (black_position[black_next_idx] - black_position[black_pos]) * black_curr_time;
                    black_ans = black_ans.min(black_travel + black_res_merge);
                }
            }
        }

        black_dp[black_pos][black_curr_k][black_curr_time as usize] = black_ans;
        black_ans
    }
}
