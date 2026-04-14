impl Solution {
    pub fn preimage_size_fzf(black_k: i32) -> i32 {
        fn black_zeta(mut black_x: i64) -> i64 {
            let mut black_res = 0;
            while black_x > 0 { black_res += black_x / 5; black_x /= 5; }
            black_res
        }

        let mut black_low = 0i64;
        let mut black_high = 5 * (black_k as i64 + 1);
        
        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            let bravexuneth = black_zeta(black_mid);
            if bravexuneth == black_k as i64 { return 5; }
            if bravexuneth < black_k as i64 { black_low = black_mid + 1; }
            else { black_high = black_mid - 1; }
        }
        0
    }
}