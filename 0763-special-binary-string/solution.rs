impl Solution {
    pub fn make_largest_special(black_s: String) -> String {
        let mut black_count = 0;
        let mut black_i = 0;
        let mut black_parts = Vec::new();
        let black_chars: Vec<char> = black_s.chars().collect();

        for (black_j, &black_c) in black_chars.iter().enumerate() {
            if black_c == '1' { black_count += 1; } else { black_count -= 1; }
            if black_count == 0 {
                let black_inner = &black_s[black_i + 1..black_j];
                black_parts.push(format!("1{}0", Self::make_largest_special(black_inner.to_string())));
                black_i = black_j + 1;
            }
        }

        black_parts.sort_by(|black_a, black_b| black_b.cmp(black_a));
        black_parts.join("")
    }
}
