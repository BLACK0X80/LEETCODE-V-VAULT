impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let black_n = nums.len();
        let mut black_less = vec![vec![0; black_n + 1]; black_n];
        for black_j in 0..black_n {
            let mut black_cnt = 0;
            for black_i in 0..black_n {
                if nums[black_i] < nums[black_j] { black_cnt += 1; }
                black_less[black_i][black_j] = black_cnt;
            }
        }

        let mut black_ans = 0i64;
        for black_j in 0..black_n {
            for black_k in black_j + 1..black_n {
                if nums[black_k] < nums[black_j] {
                    let black_i_cnt = black_less[black_j][black_k];
                    let black_l_cnt = (black_n as i32 - nums[black_j]) - (black_k as i32 - black_less[black_k][black_j]);
                    black_ans += black_i_cnt as i64 * black_l_cnt as i64;
                }
            }
        }
        black_ans
    }
}
