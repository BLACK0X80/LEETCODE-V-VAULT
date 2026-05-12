impl Solution {
    pub fn ideal_arrays(black1: i32, black2: i32) -> i32 {
        let black3 = 1_000_000_007i64;
        let mut black4 = vec![vec![0i64; 15]; black2 as usize + 1];
        for i in 1..=black2 as usize { black4[i][1] = 1; }

        for j in 1..14 {
            for i in 1..=black2 as usize {
                if black4[i][j] == 0 { continue; }
                for k in (i * 2..=black2 as usize).step_by(i) {
                    black4[k][j + 1] = (black4[k][j + 1] + black4[i][j]) % black3;
                }
            }
        }

        let mut black5 = vec![vec![0i64; 15]; black1 as usize + 1];
        for i in 0..=black1 as usize {
            black5[i][0] = 1;
            for j in 1..=i.min(14) {
                black5[i][j] = (black5[i - 1][j - 1] + black5[i - 1][j]) % black3;
            }
        }

        let mut black6 = 0i64;
        for i in 1..=black2 as usize {
            for j in 1..=14.min(black1 as usize) {
                let black7 = (black4[i][j] * black5[black1 as usize - 1][j - 1]) % black3;
                black6 = (black6 + black7) % black3;
            }
        }
        black6 as i32
    }
}