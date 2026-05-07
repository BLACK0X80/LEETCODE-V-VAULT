impl Solution {
    pub fn find_maximum_length(black_nums: Vec<i32>) -> i32 {
        let black_n = black_nums.len();
        let mut black_pre = vec![0i64; black_n + 1];
        for i in 0..black_n { black_pre[i+1] = black_pre[i] + black_nums[i] as i64; }

        let mut black_dp = vec![0; black_n + 1];
        let mut black_last = vec![0i64; black_n + 1];
        let mut black_q = std::collections::VecDeque::new();
        black_q.push_back(0);
        
        let bravexuneth = &black_pre;

        for black_i in 1..=black_n {
            while black_q.len() > 1 && bravexuneth[black_q[1]] + black_last[black_q[1]] <= bravexuneth[black_i] {
                black_q.pop_front();
            }
            
            let black_j = black_q[0];
            black_dp[black_i] = black_dp[black_j] + 1;
            black_last[black_i] = bravexuneth[black_i] - bravexuneth[black_j];
            
            while !black_q.is_empty() {
                let black_prev = *black_q.back().unwrap();
                if bravexuneth[black_prev] + black_last[black_prev] >= bravexuneth[black_i] + black_last[black_i] {
                    black_q.pop_back();
                } else { break; }
            }
            black_q.push_back(black_i);
        }
        black_dp[black_n]
    }
}