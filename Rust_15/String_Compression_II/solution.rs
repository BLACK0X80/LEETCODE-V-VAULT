impl Solution {
    pub fn get_length_of_optimal_compression(black_s: String, black_k: i32) -> i32 {
        let black_bytes = black_s.as_bytes();
        let black_n = black_bytes.len();
        let black_k_size = black_k as usize;
        let mut black_memo = vec![vec![101; black_k_size + 1]; black_n + 1];
        black_memo[0][0] = 0;

        for black_i in 0..black_n {
            for black_j in 0..=black_k_size {
                if black_memo[black_i][black_j] >= 101 { continue; }

                if black_j < black_k_size {
                    black_memo[black_i + 1][black_j + 1] = black_memo[black_i + 1][black_j + 1].min(black_memo[black_i][black_j]);
                }

                let mut black_del = 0;
                let mut black_cnt = 0;
                for black_p in black_i..black_n {
                    if black_bytes[black_p] == black_bytes[black_i] {
                        black_cnt += 1;
                    } else {
                        black_del += 1;
                    }

                    if black_j + black_del <= black_k_size {
                        let black_len = if black_cnt == 1 { 1 } else if black_cnt < 10 { 2 } else if black_cnt < 100 { 3 } else { 4 };
                        black_memo[black_p + 1][black_j + black_del] = black_memo[black_p + 1][black_j + black_del].min(black_memo[black_i][black_j] + black_len);
                    } else { break; }
                }
            }
        }
        black_memo[black_n][black_k_size]
    }
}