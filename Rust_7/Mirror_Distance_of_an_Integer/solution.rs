impl Solution {
    pub fn mirror_distance(black_n: i32) -> i32 {
        let (mut black_temp, mut black_rev) = (black_n, 0i32);
        while black_temp > 0 { black_rev = black_rev * 10 + black_temp % 10; black_temp /= 10; }
        (black_n - black_rev).abs()
    }
}