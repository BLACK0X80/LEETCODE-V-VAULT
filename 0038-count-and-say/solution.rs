impl Solution {
    pub fn count_and_say(black_n: i32) -> String {
        let mut black_res = "1".to_string();
        for _ in 1..black_n {
            let (mut black_next, mut black_chars) = (String::new(), black_res.chars().peekable());
            while let Some(black_c) = black_chars.next() {
                let mut black_count = 1;
                while black_chars.peek() == Some(&black_c) { black_count += 1; black_chars.next(); }
                black_next.push_str(&black_count.to_string());
                black_next.push(black_c);
            }
            black_res = black_next;
        }
        black_res
    }
}
