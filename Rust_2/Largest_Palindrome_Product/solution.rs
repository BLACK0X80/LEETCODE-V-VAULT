impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 { return 9; }
        let upper = 10i64.pow(n as u32) - 1;
        let lower = 10i64.pow(n as u32 - 1);
        for a in (lower..=upper).rev() {
            let s = a.to_string();
            let rev: String = s.chars().rev().collect();
            let p: i64 = format!("{}{}", s, rev).parse().unwrap();
            let mut b = upper;
            while b * b >= p {
                if p % b == 0 { return (p % 1337) as i32; }
                b -= 1;
            }
        }
        0
    }
}