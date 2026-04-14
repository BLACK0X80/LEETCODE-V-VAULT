impl Solution {
    pub fn is_valid_sudoku(black_b: Vec<Vec<char>>) -> bool {
        let mut black_seen = std::collections::HashSet::new();
        (0..9).all(|i| (0..9).all(|j| black_b[i][j] == '.' || [format!("r{}{}", i, black_b[i][j]), format!("c{}{}", j, black_b[i][j]), format!("b{}{}{}", i/3, j/3, black_b[i][j])].iter().all(|s| black_seen.insert(s.clone()))))
    }
}