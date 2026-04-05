impl Solution {
    pub fn min_moves_to_make_palindrome(s: String) -> i32 {
        let mut black_bytes = s.into_bytes();
        let mut black_res = 0;
        let mut black_l = 0;
        let mut black_r = black_bytes.len() - 1;

        while black_l < black_r {
            let mut black_k = black_r;
            while black_bytes[black_l] != black_bytes[black_k] {
                black_k -= 1;
            }
            if black_l == black_k {
                black_res += (black_bytes.len() / 2 - black_l) as i32;
            } else {
                for black_i in black_k..black_r {
                    black_bytes.swap(black_i, black_i + 1);
                    black_res += 1;
                }
                black_r -= 1;
            }
            black_l += 1;
        }
        black_res
    }
}
