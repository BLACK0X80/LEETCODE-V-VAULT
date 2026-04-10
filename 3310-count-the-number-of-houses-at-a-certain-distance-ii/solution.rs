impl Solution {
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i64> {
        let mut black_x = x as i64;
        let mut black_y = y as i64;
        if black_x > black_y {
            std::mem::swap(&mut black_x, &mut black_y);
        }

        let black_n_idx = n as usize;
        let mut black_a = vec![0i64; black_n_idx];

        for black_i in 1..=n as i64 {
            black_a[0] += 2;

            let black_dist_1 = (black_i - 1).min((black_i - black_y).abs() + black_x);
            if black_dist_1 < n as i64 {
                black_a[black_dist_1 as usize] -= 1;
            }

            let black_dist_n = (n as i64 - black_i).min((black_i - black_x).abs() + 1 + n as i64 - black_y);
            if black_dist_n < n as i64 {
                black_a[black_dist_n as usize] -= 1;
            }

            let black_split_x = (black_i - black_x).abs().min((black_y - black_i).abs() + 1);
            if black_split_x < n as i64 {
                black_a[black_split_x as usize] += 1;
            }

            let black_split_y = ((black_i - black_x).abs() + 1).min((black_y - black_i).abs());
            if black_split_y < n as i64 {
                black_a[black_split_y as usize] += 1;
            }

            let black_r = (black_x - black_i).max(0) + (black_i - black_y).max(0);
            
            let black_mid1 = black_r + (black_y - black_x + 0) / 2;
            if black_mid1 < n as i64 {
                black_a[black_mid1 as usize] -= 1;
            }

            let black_mid2 = black_r + (black_y - black_x + 1) / 2;
            if black_mid2 < n as i64 {
                black_a[black_mid2 as usize] -= 1;
            }
        }

        for black_idx in 1..black_n_idx {
            black_a[black_idx] += black_a[black_idx - 1];
        }

        black_a
    }
}
