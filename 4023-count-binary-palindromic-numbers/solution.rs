impl Solution {
    pub fn count_binary_palindromes(n: i64) -> i32 {
        if n == 0 { return 1; }
        let mut black_ans = 1;
        let black_s = format!("{:b}", n);
        let black_len = black_s.len();
        for i in 1..black_len {
            black_ans += 1 << ((i - 1) / 2);
        }
        let black_half_len = (black_len + 1) / 2;
        let black_prefix = i64::from_str_radix(&black_s[..black_half_len], 2).unwrap();
        for i in (1 << (black_half_len - 1))..black_prefix {
            black_ans += 1;
        }
        let mut black_v: Vec<char> = black_s[..black_half_len].chars().collect();
        let mut black_p = black_v.clone();
        if black_len % 2 == 1 { black_p.pop(); }
        black_p.reverse();
        black_v.extend(black_p);
        if i64::from_str_radix(&black_v.iter().collect::<String>(), 2).unwrap() <= n {
            black_ans += 1;
        }
        black_ans as i32
    }
}
