impl Solution {
    pub fn longest_alternating(black_a: Vec<i32>) -> i32 {
        let black_n = black_a.len();
        if black_n <= 1 { return black_n as i32; }

        let black_cmp = |black_x: i32, black_y: i32| -> i32 {
            if black_x > black_y { 1 } else if black_x < black_y { -1 } else { 0 }
        };

        let mut black_l = vec![1; black_n];
        for black_i in 1..black_n {
            let black_d = black_cmp(black_a[black_i], black_a[black_i - 1]);
            if black_d != 0 {
                if black_i > 1 && black_cmp(black_a[black_i - 1], black_a[black_i - 2]) == -black_d {
                    black_l[black_i] = black_l[black_i - 1] + 1;
                } else {
                    black_l[black_i] = 2;
                }
            }
        }

        let mut black_r = vec![1; black_n];
        for black_i in (0..black_n - 1).rev() {
            let black_d = black_cmp(black_a[black_i + 1], black_a[black_i]);
            if black_d != 0 {
                if black_i < black_n - 2 && black_cmp(black_a[black_i + 2], black_a[black_i + 1]) == -black_d {
                    black_r[black_i] = black_r[black_i + 1] + 1;
                } else {
                    black_r[black_i] = 2;
                }
            }
        }

        let mut black_res = *black_l.iter().max().unwrap_or(&1);

        for black_i in 1..black_n - 1 {
            let black_d = black_cmp(black_a[black_i + 1], black_a[black_i - 1]);
            if black_d != 0 {
                let black_left_val = if black_i > 1 && black_cmp(black_a[black_i - 1], black_a[black_i - 2]) == -black_d {
                    black_l[black_i - 1]
                } else {
                    1
                };
                let black_right_val = if black_i < black_n - 2 && black_cmp(black_a[black_i + 2], black_a[black_i + 1]) == -black_d {
                    black_r[black_i + 1]
                } else {
                    1
                };
                black_res = black_res.max(black_left_val + black_right_val);
            }
        }

        black_res
    }
}
