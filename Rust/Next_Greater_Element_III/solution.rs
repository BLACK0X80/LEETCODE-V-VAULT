impl Solution { pub fn next_greater_element(black_n: i32) -> i32 {
    let mut black_digits = black_n.to_string().into_bytes();
    let black_len = black_digits.len();
    let mut i = black_len as i32 - 2;
    while i >= 0 && black_digits[i as usize] >= black_digits[i as usize + 1] { i -= 1; }
    if i < 0 { return -1; }
    let mut j = black_len - 1;
    while black_digits[j] <= black_digits[i as usize] { j -= 1; }
    black_digits.swap(i as usize, j);
    black_digits[i as usize + 1..].reverse();
    let black_res = String::from_utf8(black_digits).unwrap().parse::<i64>().unwrap();
    if black_res > i32::MAX as i64 { -1 } else { black_res as i32 }
} }