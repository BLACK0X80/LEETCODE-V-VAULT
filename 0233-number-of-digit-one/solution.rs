impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let (mut black, mut b, n) = (0i64, 1i64, n as i64);
        while b <= n {
            let (hi, cur, lo) = (n/(b*10), (n/b)%10, n%b);
            black += hi * b + match cur { 0 => 0, 1 => lo+1, _ => b };
            b *= 10;
        }
        black as i32
    }
}
