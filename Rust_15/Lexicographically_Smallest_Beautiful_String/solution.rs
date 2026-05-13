impl Solution {
    pub fn smallest_beautiful_string(black_s: String, black_k: i32) -> String {
        let mut black_bytes = black_s.into_bytes();
        let black_n = black_bytes.len();
        let black_limit = b'a' + black_k as u8;
        let mut black_idx = (black_n - 1) as isize;
        
        while black_idx >= 0 {
            black_bytes[black_idx as usize] += 1;
            while black_bytes[black_idx as usize] < black_limit && 
                  ((black_idx > 0 && black_bytes[black_idx as usize] == black_bytes[black_idx as usize - 1]) || 
                   (black_idx > 1 && black_bytes[black_idx as usize] == black_bytes[black_idx as usize - 2])) {
                black_bytes[black_idx as usize] += 1;
            }
            if black_bytes[black_idx as usize] < black_limit {
                for black_i in (black_idx as usize + 1)..black_n {
                    let mut black_c = b'a';
                    while (black_i > 0 && black_c == black_bytes[black_i - 1]) || 
                          (black_i > 1 && black_c == black_bytes[black_i - 2]) {
                        black_c += 1;
                    }
                    black_bytes[black_i] = black_c;
                }
                return String::from_utf8(black_bytes).unwrap();
            }
            black_idx -= 1;
        }
        "".to_string()
    }
}