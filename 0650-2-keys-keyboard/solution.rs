impl Solution { pub fn min_steps(mut black_n: i32) -> i32 { let (mut black_ans, mut d) = (0, 2); while black_n > 1 { while black_n % d == 0 { black_ans += d; black_n /= d; } d += 1; } black_ans } }
