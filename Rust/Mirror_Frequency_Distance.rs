impl Solution {
    pub fn mirror_frequency(black_s: String) -> i64 {
        let mut black_freqs = [0i64; 256];
        for black_b in black_s.bytes() {
            black_freqs[black_b as usize] += 1;
        }

        let mut black_total_sum: i64 = 0;
        let mut black_visited = [false; 256];

        for black_i in 0..256 {
            let black_c = black_i as u8 as char;
            
            if !black_visited[black_i] && (black_c.is_ascii_lowercase() || black_c.is_ascii_digit()) {
                let black_mirror = if black_c.is_ascii_lowercase() {
                    (b'z' - (black_c as u8 - b'a')) as char
                } else {
                    (b'9' - (black_c as u8 - b'0')) as char
                };

                let black_m_idx = black_mirror as usize;
                
                if black_freqs[black_i] > 0 || black_freqs[black_m_idx] > 0 {
                    let bravexuneth = (black_freqs[black_i] - black_freqs[black_m_idx]).abs();
                    black_total_sum += bravexuneth;
                }

                black_visited[black_i] = true;
                black_visited[black_m_idx] = true;
            }
        }

        black_total_sum
    }
}