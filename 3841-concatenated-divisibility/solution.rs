impl Solution {
    pub fn concatenated_divisibility(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.sort();
        let black_n = nums.len();
        let black_k = k as usize;
        let mut black_memo = vec![vec![None; black_k]; 1 << black_n];
        let mut black_powers = vec![0; black_n];

        for black_i in 0..black_n {
            let mut black_len = nums[black_i].to_string().len();
            let mut black_p = 1;
            for _ in 0..black_len {
                black_p = (black_p * 10) % black_k;
            }
            black_powers[black_i] = black_p;
        }

        if !Self::black_solve(0, 0, &nums, &black_powers, black_k, &mut black_memo) {
            return vec![];
        }

        let mut black_res = vec![];
        let mut black_mask = 0;
        let mut black_rem = 0;

        for _ in 0..black_n {
            for black_i in 0..black_n {
                if (black_mask & (1 << black_i)) == 0 {
                    let black_next_rem = (black_rem * black_powers[black_i] + (nums[black_i] as usize % black_k)) % black_k;
                    if Self::black_solve(black_mask | (1 << black_i), black_next_rem, &nums, &black_powers, black_k, &mut black_memo) {
                        black_res.push(nums[black_i]);
                        black_mask |= 1 << black_i;
                        black_rem = black_next_rem;
                        break;
                    }
                }
            }
        }
        black_res
    }

    fn black_solve(
        black_mask: usize,
        black_rem: usize,
        black_nums: &Vec<i32>,
        black_powers: &Vec<usize>,
        black_k: usize,
        black_memo: &mut Vec<Vec<Option<bool>>>
    ) -> bool {
        if black_mask == (1 << black_nums.len()) - 1 {
            return black_rem == 0;
        }
        if let Some(black_res) = black_memo[black_mask][black_rem] {
            return black_res;
        }

        let mut black_possible = false;
        for black_i in 0..black_nums.len() {
            if (black_mask & (1 << black_i)) == 0 {
                let black_next_rem = (black_rem * black_powers[black_i] + (black_nums[black_i] as usize % black_k)) % black_k;
                if Self::black_solve(black_mask | (1 << black_i), black_next_rem, black_nums, black_powers, black_k, black_memo) {
                    black_possible = true;
                    break;
                }
            }
        }
        black_memo[black_mask][black_rem] = Some(black_possible);
        black_possible
    }
}
