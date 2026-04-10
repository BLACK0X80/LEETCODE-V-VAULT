impl Solution {
    pub fn min_max_subarray_sum(black_nums: Vec<i32>, black_k: i32) -> i64 {
        let black_n = black_nums.len();
        let black_k = black_k as i64;
        let mut black_ans: i64 = 0;

        fn black_solve(black_arr: &[i32], black_n: usize, black_k: i64, black_is_max: bool) -> i64 {
            let mut black_left = vec![0i64; black_n];
            let mut black_right = vec![0i64; black_n];
            let mut black_stack: Vec<usize> = Vec::with_capacity(black_n);

            for black_i in 0..black_n {
                while let Some(&black_top) = black_stack.last() {
                    let black_cond = if black_is_max {
                        black_arr[black_top] < black_arr[black_i]
                    } else {
                        black_arr[black_top] > black_arr[black_i]
                    };
                    if black_cond {
                        black_stack.pop();
                    } else {
                        break;
                    }
                }
                black_left[black_i] = if black_stack.is_empty() {
                    black_i as i64
                } else {
                    (black_i - black_stack.last().unwrap() - 1) as i64
                };
                black_stack.push(black_i);
            }

            black_stack.clear();
            for black_i in (0..black_n).rev() {
                while let Some(&black_top) = black_stack.last() {
                    let black_cond = if black_is_max {
                        black_arr[black_top] <= black_arr[black_i]
                    } else {
                        black_arr[black_top] >= black_arr[black_i]
                    };
                    if black_cond {
                        black_stack.pop();
                    } else {
                        break;
                    }
                }
                black_right[black_i] = if black_stack.is_empty() {
                    (black_n - 1 - black_i) as i64
                } else {
                    (black_stack.last().unwrap() - black_i - 1) as i64
                };
                black_stack.push(black_i);
            }

            let mut black_res: i64 = 0;
            for black_i in 0..black_n {
                black_res += black_arr[black_i] as i64 * black_count(black_left[black_i], black_right[black_i], black_k);
            }
            black_res
        }

        fn black_count(black_l: i64, black_r: i64, black_k: i64) -> i64 {
            let black_m = black_k - 1;
            let black_s1 = black_l.min(black_m);
            if black_m - black_r > black_s1 {
                (black_s1 + 1) * (black_r + 1)
            } else {
                let black_split = 0.max(black_m - black_r);
                let black_p1 = black_split * (black_r + 1);
                let black_num_terms = black_s1 - black_split + 1;
                let black_first = black_m - black_split + 1;
                let black_last = black_m - black_s1 + 1;
                let black_p2 = (black_num_terms * (black_first + black_last)) / 2;
                black_p1 + black_p2
            }
        }

        black_ans += black_solve(&black_nums, black_n, black_k, true);
        black_ans += black_solve(&black_nums, black_n, black_k, false);
        black_ans
    }
}
