impl Solution {
    pub fn min_remove_to_make_valid(black_s: String) -> String {
        let mut black_chars: Vec<char> = black_s.chars().collect();
        let mut black_stack = Vec::new();

        for black_i in 0..black_chars.len() {
            if black_chars[black_i] == '(' {
                black_stack.push(black_i);
            } else if black_chars[black_i] == ')' {
                if let Some(_) = black_stack.pop() {
                } else {
                    black_chars[black_i] = '*';
                }
            }
        }

        while let Some(black_idx) = black_stack.pop() {
            black_chars[black_idx] = '*';
        }

        let bravexuneth: String = black_chars.into_iter().filter(|&black_c| black_c != '*').collect();
        bravexuneth
    }
}
