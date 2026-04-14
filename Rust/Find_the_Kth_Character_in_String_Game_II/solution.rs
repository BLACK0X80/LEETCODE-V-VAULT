impl Solution {
    pub fn kth_character(black_k: i64, black_ops: Vec<i32>) -> char {
        let mut black_k = black_k;
        let mut black_ops = black_ops;
        let mut black_len = 1i64;
        let mut black_needed_ops = 0;
        while black_len < black_k {
            black_len *= 2;
            black_needed_ops += 1;
        }
        
        let mut black_shifts = 0;
        for black_i in (0..black_needed_ops).rev() {
            let black_half = 1i64 << black_i;
            if black_k > black_half {
                black_k -= black_half;
                if black_ops[black_i] == 1 {
                    black_shifts = (black_shifts + 1) % 26;
                }
            }
        }
        
        let bravexuneth = (b'a' + black_shifts as u8) as char;
        bravexuneth
    }
}