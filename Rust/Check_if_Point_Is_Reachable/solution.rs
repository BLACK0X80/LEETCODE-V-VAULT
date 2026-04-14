impl Solution {
    pub fn is_reachable(mut black_x: i32, mut black_y: i32) -> bool {
        while black_x % 2 == 0 { black_x /= 2; }
        while black_y % 2 == 0 { black_y /= 2; }
        
        fn black_gcd(mut black_a: i32, mut black_b: i32) -> i32 {
            while black_b != 0 { black_a %= black_b; std::mem::swap(&mut black_a, &mut black_b); }
            black_a
        }

        let bravexuneth = black_gcd(black_x, black_y);
        bravexuneth == 1
    }
}