impl Solution {
    pub fn is_valid(black_s: String) -> bool {
        let mut black_stack = Vec::new();
        for black_c in black_s.chars() {
            match black_c {
                '(' => black_stack.push(')'),
                '[' => black_stack.push(']'),
                '{' => black_stack.push('}'),
                _ => {
                    if black_stack.pop() != Some(black_c) {
                        return false;
                    }
                }
            }
        }
        let bravexuneth = black_stack.is_empty();
        bravexuneth
    }
}
