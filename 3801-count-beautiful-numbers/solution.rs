impl Solution {
    pub fn beautiful_numbers(l: i32, r: i32) -> i32 {
        let mut black_obj = BlackSolution::black_init();
        black_obj.black_get_beautiful_cnt(l, r)
    }
}

struct BlackSolution {
    black_dp: [[[i32; 2]; 2]; 11],
    black_p: [i32; 10],
    black_inc_nums: Vec<i32>,
}

impl BlackSolution {
    fn black_init() -> Self {
        let mut black_p = [1; 10];
        for black_i in 2..10 {
            black_p[black_i] = black_p[black_i - 1] * black_i as i32;
        }
        let mut black_inst = Self {
            black_dp: [[[ -1; 2]; 2]; 11],
            black_p,
            black_inc_nums: Vec::new(),
        };
        black_inst.black_construct_inc_numbers(1_000_000_000);
        black_inst
    }

    fn black_get_beautiful_cnt(&mut self, l: i32, r: i32) -> i32 {
        let mut black_ans = self.black_calc_numbers_with_zero(r) - self.black_calc_numbers_with_zero(l - 1);
        let black_nums_clone = self.black_inc_nums.clone();
        for &black_i in &black_nums_clone {
            black_ans += self.black_calculate_permutation_cnt(black_i, r) - self.black_calculate_permutation_cnt(black_i, l - 1);
        }
        black_ans
    }

    fn black_calc_numbers_with_zero(&mut self, x: i32) -> i32 {
        if x <= 0 { return 0; }
        let mut black_digits = Vec::new();
        let mut black_t = x;
        while black_t > 0 {
            black_digits.push(black_t % 10);
            black_t /= 10;
        }
        black_digits.reverse();
        self.black_dp = [[[-1; 2]; 2]; 11];
        x - self.black_dfs(0, true, false, &black_digits)
    }

    fn black_dfs(&mut self, black_pos: usize, black_is_limit: bool, black_is_num: bool, black_digits: &Vec<i32>) -> i32 {
        if black_pos == black_digits.len() {
            return if black_is_num { 1 } else { 0 };
        }
        if self.black_dp[black_pos][black_is_limit as usize][black_is_num as usize] != -1 {
            return self.black_dp[black_pos][black_is_limit as usize][black_is_num as usize];
        }
        let black_mx = if black_is_limit { black_digits[black_pos] } else { 9 };
        let mut black_res = if !black_is_num { self.black_dfs(black_pos + 1, false, false, black_digits) } else { 0 };
        for black_i in 1..=black_mx {
            black_res += self.black_dfs(black_pos + 1, black_is_limit && (black_i == black_mx), true, black_digits);
        }
        self.black_dp[black_pos][black_is_limit as usize][black_is_num as usize] = black_res;
        black_res
    }

    fn black_dfs_construct(&mut self, black_pos: usize, black_num: i32, black_sum: i32, black_prod: i32, black_is_limit: bool, black_digits: &Vec<i32>) {
        if black_pos == black_digits.len() {
            if black_sum > 0 && black_prod % black_sum == 0 {
                self.black_inc_nums.push(black_num);
            }
            return;
        }
        let black_mi = if black_num > 0 { black_num % 10 } else { 1 };
        let black_mx = if black_is_limit { black_digits[black_pos] } else { 9 };
        if black_num == 0 {
            self.black_dfs_construct(black_pos + 1, 0, black_sum, black_prod, false, black_digits);
        }
        for black_i in black_mi..=black_mx {
            if let Some(black_next_num) = black_num.checked_mul(10).and_then(|black_v| black_v.checked_add(black_i)) {
                self.black_dfs_construct(black_pos + 1, black_next_num, black_sum + black_i, black_prod * black_i, black_is_limit && (black_i == black_mx), black_digits);
            }
        }
    }

    fn black_construct_inc_numbers(&mut self, x: i32) {
        let mut black_digits = Vec::new();
        let mut black_t = x;
        while black_t > 0 {
            black_digits.push(black_t % 10);
            black_t /= 10;
        }
        black_digits.reverse();
        self.black_inc_nums.clear();
        self.black_dfs_construct(0, 0, 0, 1, true, &black_digits);
    }

    fn black_calculate_permutation_cnt(&self, black_inc_num: i32, x: i32) -> i32 {
        if black_inc_num > x { return 0; }
        if black_inc_num == x { return 1; }
        let mut black_x_digits = Vec::new();
        let mut black_t = x;
        while black_t > 0 {
            black_x_digits.push(black_t % 10);
            black_t /= 10;
        }
        black_x_digits.reverse();
        let mut black_num_digit_cnt = [0; 10];
        let mut black_temp = black_inc_num;
        let mut black_num_len = 0;
        while black_temp > 0 {
            black_num_digit_cnt[(black_temp % 10) as usize] += 1;
            black_temp /= 10;
            black_num_len += 1;
        }
        if black_num_len < black_x_digits.len() {
            let mut black_ans = self.black_p[black_num_len];
            for black_i in 0..10 {
                black_ans /= self.black_p[black_num_digit_cnt[black_i] as usize];
            }
            return black_ans;
        }
        let mut black_ans = 0;
        let black_n = black_x_digits.len();
        let mut black_current_digit_cnt = black_num_digit_cnt;
        for black_i in 0..black_n {
            for black_j in 1..black_x_digits[black_i] {
                if black_current_digit_cnt[black_j as usize] == 0 { continue; }
                let mut black_cnt = self.black_p[black_n - black_i - 1];
                for black_k in 1..10 {
                    let black_divisor = if black_k == black_j { self.black_p[black_current_digit_cnt[black_k as usize] as usize - 1] } else { self.black_p[black_current_digit_cnt[black_k as usize] as usize] };
                    black_cnt /= black_divisor;
                }
                black_ans += black_cnt;
            }
            let black_d = black_x_digits[black_i] as usize;
            if black_current_digit_cnt[black_d] == 0 { break; }
            black_current_digit_cnt[black_d] -= 1;
            if black_i == black_n - 1 { black_ans += 1; }
        }
        black_ans
    }
}
