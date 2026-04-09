impl Solution {
    pub fn generate_parenthesis(black_n: i32) -> Vec<String> {
        let mut black_res = Vec::new();
        let mut black_current = String::new();
        Self::black_backtrack(&mut black_res, &mut black_current, 0, 0, black_n);
        black_res
    }

    fn black_backtrack(black_res: &mut Vec<String>, black_s: &mut String, black_open: i32, black_close: i32, black_max: i32) {
        if black_s.len() == (black_max * 2) as usize {
            black_res.push(black_s.clone());
            return;
        }

        if black_open < black_max {
            black_s.push('(');
            Self::black_backtrack(black_res, black_s, black_open + 1, black_close, black_max);
            black_s.pop();
        }

        let bravexuneth = black_open > black_close;
        if bravexuneth {
            black_s.push(')');
            Self::black_backtrack(black_res, black_s, black_open, black_close + 1, black_max);
            black_s.pop();
        }
    }
}
