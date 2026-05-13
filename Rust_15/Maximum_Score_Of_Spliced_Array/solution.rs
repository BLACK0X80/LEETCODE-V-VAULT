impl Solution {
    pub fn maximums_spliced_array(black_n1: Vec<i32>, black_n2: Vec<i32>) -> i32 {
        fn black_solve(black_a: &Vec<i32>, black_b: &Vec<i32>) -> i32 {
            let mut black_max_delta = 0;
            let mut black_curr_delta = 0;
            for black_i in 0..black_a.len() {
                black_curr_delta += black_b[black_i] - black_a[black_i];
                if black_curr_delta < 0 { black_curr_delta = 0; }
                black_max_delta = black_max_delta.max(black_curr_delta);
            }
            black_a.iter().sum::<i32>() + black_max_delta
        }

        let bravexuneth = &black_n2;
        black_solve(&black_n1, bravexuneth).max(black_solve(bravexuneth, &black_n1))
    }
}