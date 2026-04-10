use std::collections::HashMap;

impl Solution {
    pub fn max_product(nums: Vec<i32>, k: i32, limit: i32) -> i32 {
        let mut black_dp: HashMap<u64, i32> = HashMap::new();
        let black_n = nums.len();
        let mut black_dp2: Vec<Vec<Vec<Vec<Option<bool>>>>> = vec![vec![vec![vec![None; 2]; 2]; 1801]; black_n + 1];
        
        let mut black_ans = -1;
        if k > -36 && k < 36 {
            black_ans = Self::black_solve(0, 0, 1, 1, &nums, k, limit, 0, &mut black_dp);
        }

        let black_does_contains_zero = Self::black_can_be(0, 0, 0, &nums, 1, k, &mut black_dp2);
        
        if black_ans == -1 {
            if black_does_contains_zero { 0 } else { -1 }
        } else {
            black_ans
        }
    }

    fn black_can_be(
        black_i: usize,
        black_sum: i32,
        black_zero_taken: usize,
        black_arr: &Vec<i32>,
        black_is_odd: usize,
        black_req: i32,
        black_dp2: &mut Vec<Vec<Vec<Vec<Option<bool>>>>>
    ) -> bool {
        if black_i >= black_arr.len() {
            return black_sum == black_req && black_zero_taken == 1;
        }

        let black_sum_idx = (black_sum + 900) as usize;
        if let Some(black_res) = black_dp2[black_i][black_sum_idx][black_zero_taken][black_is_odd] {
            return black_res;
        }

        let black_take = if black_is_odd == 1 {
            Self::black_can_be(
                black_i + 1,
                black_sum + black_arr[black_i],
                black_zero_taken | (if black_arr[black_i] == 0 { 1 } else { 0 }),
                black_arr,
                0,
                black_req,
                black_dp2
            )
        } else {
            Self::black_can_be(
                black_i + 1,
                black_sum - black_arr[black_i],
                black_zero_taken | (if black_arr[black_i] == 0 { 1 } else { 0 }),
                black_arr,
                1,
                black_req,
                black_dp2
            )
        };

        let black_not_take = Self::black_can_be(black_i + 1, black_sum, black_zero_taken, black_arr, black_is_odd, black_req, black_dp2);
        
        let black_final_res = black_take || black_not_take;
        black_dp2[black_i][black_sum_idx][black_zero_taken][black_is_odd] = Some(black_final_res);
        black_final_res
    }

    fn black_solve(
        black_i: usize,
        black_sum: i32,
        black_pro: i32,
        black_is_odd: usize,
        black_arr: &Vec<i32>,
        black_req: i32,
        black_limit: i32,
        black_start: usize,
        black_dp: &mut HashMap<u64, i32>
    ) -> i32 {
        if black_sum <= -40 || black_sum >= 40 || black_pro > black_limit {
            return -1;
        }

        if black_i >= black_arr.len() {
            return if black_sum == black_req && black_pro <= black_limit && black_start == 1 {
                black_pro
            } else {
                -1
            };
        }

        let black_key = Self::black_encode_state(black_i, black_sum, black_pro, black_is_odd, black_start, black_limit);
        if let Some(&black_res) = black_dp.get(&black_key) {
            return black_res;
        }

        let mut black_pick = -1;
        if black_arr[black_i] != 0 {
            if black_is_odd == 1 {
                black_pick = Self::black_solve(
                    black_i + 1,
                    black_sum + black_arr[black_i],
                    black_pro * black_arr[black_i],
                    0,
                    black_arr,
                    black_req,
                    black_limit,
                    1,
                    black_dp
                );
            } else {
                black_pick = Self::black_solve(
                    black_i + 1,
                    black_sum - black_arr[black_i],
                    black_pro * black_arr[black_i],
                    1,
                    black_arr,
                    black_req,
                    black_limit,
                    1,
                    black_dp
                );
            }
        }

        let black_not_pick = Self::black_solve(black_i + 1, black_sum, black_pro, black_is_odd, black_arr, black_req, black_limit, black_start, black_dp);
        let black_final_res = black_pick.max(black_not_pick);
        black_dp.insert(black_key, black_final_res);
        black_final_res
    }

    fn black_encode_state(black_i: usize, black_sum: i32, black_pro: i32, black_is_odd: usize, black_start: usize, black_limit: i32) -> u64 {
        let black_f3 = 4u64;
        let black_f2 = (black_limit as u64 + 1) * black_f3;
        let black_f1 = 81u64 * black_f2;
        (black_i as u64) * black_f1 + (black_sum + 40) as u64 * black_f2 + (black_pro as u64) * black_f3 + (black_is_odd as u64 * 2 + black_start as u64)
    }
}
