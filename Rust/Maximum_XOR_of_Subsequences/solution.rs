impl Solution {
    pub fn max_xor_subsequences(black1: Vec<i32>) -> i32 {
        let mut black2 = vec![0; 31];
        for mut black3 in black1 {
            for i in (0..31).rev() {
                if (black3 >> i) & 1 == 1 {
                    if black2[i] == 0 { black2[i] = black3; break; }
                    black3 ^= black2[i];
                }
            }
        }
        let mut black4 = 0;
        for i in (0..31).rev() {
            if (black4 ^ black2[i]) > black4 { black4 ^= black2[i]; }
        }
        black4
    }
}