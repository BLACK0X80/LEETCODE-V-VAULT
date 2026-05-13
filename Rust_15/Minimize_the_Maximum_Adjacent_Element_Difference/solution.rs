impl Solution {
    pub fn min_difference(black_nums: Vec<i32>) -> i32 {
        let mut black_max_gap = 0;
        let mut black_min_n = i32::MAX;
        let mut black_max_n = 0;
        let black_len = black_nums.len();

        for black_i in 0..black_len - 1 {
            let black_a = black_nums[black_i];
            let black_b = black_nums[black_i + 1];

            if (black_a == -1 && black_b != -1) || (black_a != -1 && black_b == -1) {
                let black_val = if black_a != -1 { black_a } else { black_b };
                black_min_n = black_min_n.min(black_val);
                black_max_n = black_max_n.max(black_val);
            } else if black_a != -1 && black_b != -1 {
                black_max_gap = black_max_gap.max((black_a - black_b).abs());
            }
        }

        if black_max_n == 0 {
            return black_max_gap;
        }

        let mut black_l = black_max_gap;
        let mut black_r = (black_max_n - black_min_n + 1) / 2;

        while black_l < black_r {
            let black_d = black_l + (black_r - black_l) / 2;
            if Self::black_check_diff(&black_nums, black_min_n + black_d, black_max_n - black_d, black_d) {
                black_r = black_d;
            } else {
                black_l = black_d + 1;
            }
        }
        black_l
    }

    fn black_check_diff(black_n_vec: &Vec<i32>, black_x: i32, black_y: i32, black_d: i32) -> bool {
        let mut black_cnt = 0;
        let mut black_prev = 0;
        let mut black_has_prev = false;

        for &black_val in black_n_vec {
            if black_val == -1 {
                if black_has_prev && (black_prev - black_x).abs().min((black_prev - black_y).abs()) > black_d {
                    return false;
                }
                black_cnt += 1;
            } else {
                if black_cnt > 0 {
                    let black_diff = if black_has_prev {
                        let black_opt1 = (black_prev - black_x).abs().max((black_val - black_x).abs());
                        let black_opt2 = (black_prev - black_y).abs().max((black_val - black_y).abs());
                        black_opt1.min(black_opt2)
                    } else {
                        (black_val - black_x).abs().min((black_val - black_y).abs())
                    };

                    if black_diff > black_d && (black_cnt == 1 || black_y - black_x > black_d) {
                        return false;
                    }
                }
                black_prev = black_val;
                black_has_prev = true;
                black_cnt = 0;
            }
        }
        true
    }
}