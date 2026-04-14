use std::cmp::min;

impl Solution {
    pub fn tiling_rectangle(black_n: i32, black_m: i32) -> i32 {
        let mut black_rect = vec![vec![false; black_m as usize]; black_n as usize];
        let mut black_ans = (black_n * black_m) as i32;

        fn black_solve(
            black_r: usize,
            black_c: usize,
            black_n: usize,
            black_m: usize,
            black_rect: &mut Vec<Vec<bool>>,
            black_count: i32,
            black_ans: &mut i32,
        ) {
            if black_count >= *black_ans {
                return;
            }

            let mut black_nr = black_r;
            let mut black_nc = black_c;
            let mut black_found = false;

            for i in black_r..black_n {
                for j in 0..black_m {
                    if !black_rect[i][j] {
                        black_nr = i;
                        black_nc = j;
                        black_found = true;
                        break;
                    }
                }
                if black_found { break; }
            }

            if !black_found {
                *black_ans = min(*black_ans, black_count);
                return;
            }

            let mut black_max_s = min(black_n - black_nr, black_m - black_nc);
            for black_s in (1..=black_max_s).rev() {
                if black_can_place(black_nr, black_nc, black_s, black_rect) {
                    black_place(black_nr, black_nc, black_s, black_rect, true);
                    black_solve(black_nr, black_nc, black_n, black_m, black_rect, black_count + 1, black_ans);
                    black_place(black_nr, black_nc, black_s, black_rect, false);
                }
            }
        }

        fn black_can_place(black_r: usize, black_c: usize, black_s: usize, black_rect: &Vec<Vec<bool>>) -> bool {
            for i in black_r..black_r + black_s {
                for j in black_c..black_c + black_s {
                    if black_rect[i][j] { return false; }
                }
            }
            true
        }

        fn black_place(black_r: usize, black_c: usize, black_s: usize, black_rect: &mut Vec<Vec<bool>>, black_val: bool) {
            for i in black_r..black_r + black_s {
                for j in black_c..black_c + black_s {
                    black_rect[i][j] = black_val;
                }
            }
        }

        black_solve(0, 0, black_n as usize, black_m as usize, &mut black_rect, 0, &mut black_ans);
        black_ans
    }
}