use std::collections::HashSet;

impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        let black_total = (k as usize).pow(n as u32);
        let mut black_visited: HashSet<String> = HashSet::new();
        let mut black_result = "0".repeat(n as usize);
        black_visited.insert(black_result.clone());

        while black_visited.len() < black_total {
            let black_prefix = &black_result[black_result.len() - (n as usize - 1)..];
            let mut black_found = false;
            for black_d in (0..k).rev() {
                let black_next = format!("{}{}", black_prefix, black_d);
                if !black_visited.contains(&black_next) {
                    black_visited.insert(black_next.clone());
                    black_result.push(char::from_digit(black_d as u32, 10).unwrap());
                    black_found = true;
                    break;
                }
            }
            if !black_found { break; }
        }

        black_result
    }
}