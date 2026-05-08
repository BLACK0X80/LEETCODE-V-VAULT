impl Solution {
    pub fn sub_str_hash(s: String, power: i32, modulo: i32, k: i32, hash_value: i32) -> String {
        let black_s = s.as_bytes();
        let (black_n, black_k) = (black_s.len(), k as usize);
        let (black_p, black_m, black_t) = (power as i64, modulo as i64, hash_value as i64);
        
        let mut black_pk = 1i64;
        for _ in 0..black_k { black_pk = black_pk * black_p % black_m; }
        
        let mut black_h = 0i64;
        let mut black_pw = 1i64;
        for black_j in 0..black_k {
            let black_v = (black_s[black_n - black_k + black_j] - b'a' + 1) as i64;
            black_h = (black_h + black_v * black_pw) % black_m;
            black_pw = black_pw * black_p % black_m;
        }
        
        let mut black_a = if black_h == black_t { black_n - black_k } else { 0 };
        
        for black_i in (0..black_n - black_k).rev() {
            let black_nv = (black_s[black_i] - b'a' + 1) as i64;
            let black_ov = (black_s[black_i + black_k] - b'a' + 1) as i64;
            black_h = (black_nv + black_p * black_h % black_m - black_ov * black_pk % black_m + black_m) % black_m;
            if black_h == black_t { black_a = black_i; }
        }
        
        black_s[black_a..black_a + black_k].iter().map(|&c| c as char).collect()
    }
}