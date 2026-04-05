impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let black_s1 = s1.as_bytes();
        let black_s2 = s2.as_bytes();
        let black_len1 = black_s1.len();
        let black_len2 = black_s2.len();
        if black_len1 == 0 || black_len2 == 0 || n1 == 0 || n2 == 0 {
            return 0;
        }
        let mut black_counts = vec![0; black_len2 + 1];
        let mut black_indices = vec![0; black_len2 + 1];
        let mut black_count = 0;
        let mut black_index = 0;
        for black_i in 1..=(n1 as usize) {
            for black_j in 0..black_len1 {
                if black_s1[black_j] == black_s2[black_index] {
                    black_index += 1;
                    if black_index == black_len2 {
                        black_index = 0;
                        black_count += 1;
                    }
                }
            }
            black_counts[black_i] = black_count;
            black_indices[black_i] = black_index;
            for black_k in 0..black_i {
                if black_indices[black_k] == black_index {
                    let black_prefix_count = black_counts[black_k];
                    let black_pattern_count = (black_counts[black_i] - black_counts[black_k]) * ((n1 as usize - black_k) / (black_i - black_k));
                    let black_suffix_count = black_counts[black_k + (n1 as usize - black_k) % (black_i - black_k)] - black_counts[black_k];
                    return ((black_prefix_count + black_pattern_count + black_suffix_count) as i32) / n2;
                }
            }
        }
        black_counts[n1 as usize] as i32 / n2
    }
}
