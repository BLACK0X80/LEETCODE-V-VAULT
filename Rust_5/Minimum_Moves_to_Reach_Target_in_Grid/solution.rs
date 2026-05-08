impl Solution {
    pub fn min_moves(black_sx: i32, black_sy: i32, black_tx: i32, black_ty: i32) -> i32 {
        fn black_previ(black_x: i32, black_y: i32) -> Vec<(i32, i32)> {
            let mut black_can = Vec::new();
            if black_x == black_y {
                black_can.push((0, black_y));
                black_can.push((black_x, 0));
                return black_can;
            }
            if black_y < black_x {
                if black_x % 2 == 0 {
                    if black_x / 2 >= black_y {
                        black_can.push((black_x / 2, black_y));
                    } else if black_x - black_y <= black_y {
                        black_can.push((black_x - black_y, black_y));
                    }
                } else if black_x - black_y <= black_y {
                    black_can.push((black_x - black_y, black_y));
                }
            }
            if black_x < black_y {
                if black_y % 2 == 0 {
                    if black_y / 2 >= black_x {
                        black_can.push((black_x, black_y / 2));
                    } else if black_y - black_x <= black_x {
                        black_can.push((black_x, black_y - black_x));
                    }
                } else if black_y - black_x <= black_x {
                    black_can.push((black_x, black_y - black_x));
                }
            }
            black_can
        }

        fn black_dfs(black_sx: i32, black_sy: i32, black_a: i32, black_b: i32) -> i32 {
            if black_a < black_sx || black_b < black_sy {
                return 1_000_000_000;
            }
            if black_a == black_sx && black_b == black_sy {
                return 0;
            }
            let black_pre = black_previ(black_a, black_b);
            let mut black_ans = 1_000_000_000;
            for (black_x, black_y) in black_pre {
                black_ans = black_ans.min(1 + black_dfs(black_sx, black_sy, black_x, black_y));
            }
            black_ans
        }

        let black_res = black_dfs(black_sx, black_sy, black_tx, black_ty);
        if black_res >= 1_000_000_000 { -1 } else { black_res }
    }
}