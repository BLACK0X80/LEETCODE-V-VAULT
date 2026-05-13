impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut black_res = Vec::new();
        let mut black_path = Vec::new();
        Self::black_dfs(&s, 0, &mut black_path, &mut black_res);
        black_res
    }

    fn black_dfs(black_s: &str, black_start: usize, black_path: &mut Vec<String>, black_res: &mut Vec<Vec<String>>) {
        if black_start == black_s.len() {
            black_res.push(black_path.clone());
            return;
        }
        for black_end in black_start + 1..=black_s.len() {
            let black_sub = &black_s[black_start..black_end];
            if Self::black_is_pali(black_sub) {
                black_path.push(black_sub.to_string());
                Self::black_dfs(black_s, black_end, black_path, black_res);
                black_path.pop();
            }
        }
    }

    fn black_is_pali(black_sub: &str) -> bool {
        let black_b = black_sub.as_bytes();
        let (mut black_l, mut black_r) = (0, black_b.len() as i32 - 1);
        while black_l < black_r {
            if black_b[black_l as usize] != black_b[black_r as usize] { return false; }
            black_l += 1;
            black_r -= 1;
        }
        true
    }
}