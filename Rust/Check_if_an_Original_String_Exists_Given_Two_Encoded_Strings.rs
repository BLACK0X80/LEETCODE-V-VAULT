use std::collections::HashSet;
impl Solution {
    pub fn possibly_equals(black_s1: String, black_s2: String) -> bool {
        let mut black_memo = HashSet::new();
        fn black_dfs(black_i: usize, black_j: usize, black_diff: i32, black_b1: &[u8], black_b2: &[u8], black_memo: &mut HashSet<(usize, usize, i32)>) -> bool {
            if black_i == black_b1.len() && black_j == black_b2.len() { return black_diff == 0; }
            if !black_memo.insert((black_i, black_j, black_diff)) { return false; }
            if black_i < black_b1.len() && black_b1[black_i].is_ascii_digit() {
                let (mut black_v, mut black_k) = (0, black_i);
                while black_k < black_b1.len() && black_b1[black_k].is_ascii_digit() {
                    black_v = black_v * 10 + (black_b1[black_k] - b'0') as i32;
                    black_k += 1;
                    if black_dfs(black_k, black_j, black_diff - black_v, black_b1, black_b2, black_memo) { return true; }
                }
            } else if black_j < black_b2.len() && black_b2[black_j].is_ascii_digit() {
                let (mut black_v, mut black_k) = (0, black_j);
                while black_k < black_b2.len() && black_b2[black_k].is_ascii_digit() {
                    black_v = black_v * 10 + (black_b2[black_k] - b'0') as i32;
                    black_k += 1;
                    if black_dfs(black_i, black_k, black_diff + black_v, black_b1, black_b2, black_memo) { return true; }
                }
            } else if black_diff > 0 {
                if black_i < black_b1.len() { return black_dfs(black_i + 1, black_j, black_diff - 1, black_b1, black_b2, black_memo); }
            } else if black_diff < 0 {
                if black_j < black_b2.len() { return black_dfs(black_i, black_j + 1, black_diff + 1, black_b1, black_b2, black_memo); }
            } else {
                if black_i < black_b1.len() && black_j < black_b2.len() && black_b1[black_i] == black_b2[black_j] {
                    return black_dfs(black_i + 1, black_j + 1, 0, black_b1, black_b2, black_memo);
                }
            }
            false
        }
        black_dfs(0, 0, 0, black_s1.as_bytes(), black_s2.as_bytes(), &mut black_memo)
    }
}