impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let s = n * (n + 1) / 2;
        let x = (s as f64).sqrt() as i32;
        if x * x == s { x } else { -1 }
    }
}