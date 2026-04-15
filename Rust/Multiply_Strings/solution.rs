impl Solution { pub fn multiply(b1: String, b2: String) -> String {
    if b1 == "0" || b2 == "0" { return "0".into() } let mut r = vec![0; b1.len() + b2.len()];
    b1.bytes().rev().enumerate().for_each(|(i, x)| b2.bytes().rev().enumerate().for_each(|(j, y)| { let p = (x - 48) as i32 * (y - 48) as i32 + r[i+j]; r[i+j] = p % 10; r[i+j+1] += p / 10; }));
    while r.len() > 1 && r.last() == Some(&0) { r.pop(); } r.iter().rev().map(|&x| (x + 48) as u8 as char).collect()
}}