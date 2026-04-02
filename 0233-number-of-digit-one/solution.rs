impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut count = 0i64;
        let mut factor = 1i64;
        let n = n as i64;

        while factor <= n {
            let higher = n / (factor * 10);
            let current = (n / factor) % 10;
            let lower = n % factor;

            count += higher * factor;
            count += match current {
                0 => 0,
                1 => lower + 1,
                _ => factor,
            };

            factor *= 10;
        }

        count as i32
    }
}
