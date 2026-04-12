impl Solution {
    pub fn letter_combinations(black_digits: String) -> Vec<String> {
        if black_digits.is_empty() { return vec![]; }
        let black_map = vec!["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut black_res = vec!["".to_string()];
        for black_d in black_digits.bytes() {
            let mut black_temp = vec![];
            for black_s in &black_res {
                for black_c in black_map[(black_d - b'0') as usize].chars() {
                    black_temp.push(format!("{}{}", black_s, black_c));
                }
            }
            black_res = black_temp;
        }
        black_res
    }
}
