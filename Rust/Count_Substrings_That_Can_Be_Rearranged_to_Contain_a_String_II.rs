impl Solution {
    pub fn valid_substring_count(black_word1: String, black_word2: String) -> i64 {
        let mut black_target = [0; 26];
        for &black_b in black_word2.as_bytes() { black_target[(black_b - b'a') as usize] += 1; }
        let mut black_required = black_target.iter().filter(|&&black_c| black_c > 0).count();
        let mut black_count = [0; 26];
        let (mut black_l, mut black_res) = (0, 0i64);
        let black_bytes = black_word1.as_bytes();

        for black_r in 0..black_bytes.len() {
            let black_idx = (black_bytes[black_r] - b'a') as usize;
            black_count[black_idx] += 1;
            if black_count[black_idx] == black_target[black_idx] { black_required -= 1; }
            while black_required == 0 {
                black_res += (black_bytes.len() - black_r) as i64;
                let black_l_idx = (black_bytes[black_l] - b'a') as usize;
                if black_count[black_l_idx] == black_target[black_l_idx] { black_required += 1; }
                black_count[black_l_idx] -= 1;
                black_l += 1;
            }
        }
        black_res
    }
}