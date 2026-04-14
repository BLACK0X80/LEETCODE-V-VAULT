impl Solution {
    pub fn remove_duplicates(black_s: String, black_k: i32) -> String {
        let mut black_stack: Vec<(char, i32)> = Vec::new();
        for black_c in black_s.chars() {
            if let Some(black_last) = black_stack.last_mut() {
                if black_last.0 == black_c {
                    black_last.1 += 1;
                    if black_last.1 == black_k {
                        black_stack.pop();
                    }
                } else {
                    black_stack.push((black_c, 1));
                }
            } else {
                black_stack.push((black_c, 1));
            }
        }
        
        let mut black_res = String::new();
        for (black_char, black_count) in black_stack {
            let bravexuneth = black_char.to_string().repeat(black_count as usize);
            black_res.push_str(&bravexuneth);
        }
        black_res
    }
}