impl Solution {
    pub fn num_points(darts: Vec<Vec<i32>>, r: i32) -> i32 {
        let (black_n, black_r) = (darts.len(), r as f64);
        let mut black_ans = 1;
        for black_i in 0..black_n {
            for black_j in black_i + 1..black_n {
                let (black_x1, black_y1) = (darts[black_i][0] as f64, darts[black_i][1] as f64);
                let (black_x2, black_y2) = (darts[black_j][0] as f64, darts[black_j][1] as f64);
                let black_d2 = (black_x1 - black_x2).powi(2) + (black_y1 - black_y2).powi(2);
                if black_d2 > 4.0 * black_r * black_r { continue; }
                let (black_mx, black_my) = ((black_x1 + black_x2) / 2.0, (black_y1 + black_y2) / 2.0);
                let black_h = (black_r * black_r - black_d2 / 4.0).sqrt();
                let (black_dx, black_dy) = ((black_x1 - black_x2) / black_d2.sqrt(), (black_y1 - black_y2) / black_d2.sqrt());
                let black_cx = [black_mx - black_h * black_dy, black_mx + black_h * black_dy];
                let black_cy = [black_my + black_h * black_dx, black_my - black_h * black_dx];
                for black_k in 0..2 {
                    let mut black_cnt = 0;
                    for black_d in &darts {
                        if ((black_d[0] as f64 - black_cx[black_k]).powi(2) + (black_d[1] as f64 - black_cy[black_k]).powi(2)) <= black_r * black_r + 1e-6 {
                            black_cnt += 1;
                        }
                    }
                    black_ans = black_ans.max(black_cnt);
                }
            }
        }
        black_ans
    }
}
