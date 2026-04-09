impl Solution {
    pub fn judge_point24(black_cards: Vec<i32>) -> bool {
        let mut black_nums: Vec<f64> = black_cards.into_iter().map(|black_x| black_x as f64).collect();
        Self::black_solve(&mut black_nums)
    }

    fn black_solve(black_nums: &mut Vec<f64>) -> bool {
        let black_n = black_nums.len();
        if black_n == 1 {
            return (black_nums[0] - 24.0).abs() < 1e-6;
        }

        for black_i in 0..black_n {
            for black_j in 0..black_n {
                if black_i == black_j { continue; }

                let mut black_next_nums = Vec::new();
                for black_k in 0..black_n {
                    if black_k != black_i && black_k != black_j {
                        black_next_nums.push(black_nums[black_k]);
                    }
                }

                for black_op in 0..4 {
                    if black_op < 2 && black_i > black_j { continue; }
                    
                    match black_op {
                        0 => black_next_nums.push(black_nums[black_i] + black_nums[black_j]),
                        1 => black_next_nums.push(black_nums[black_i] * black_nums[black_j]),
                        2 => black_next_nums.push(black_nums[black_i] - black_nums[black_j]),
                        3 => {
                            if black_nums[black_j].abs() < 1e-9 { continue; }
                            black_next_nums.push(black_nums[black_i] / black_nums[black_j]);
                        },
                        _ => unreachable!(),
                    }

                    if Self::black_solve(&mut black_next_nums) { return true; }
                    black_next_nums.pop();
                }
            }
        }
        false
    }
}
