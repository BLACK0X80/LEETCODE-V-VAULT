impl Solution {
    pub fn smallest_palindrome(s: String, black_k: i32) -> String {
        let mut black_c = [0i64; 26];
        for &black_b in s.as_bytes() { black_c[(black_b - b'a') as usize] += 1; }
        let (mut black_o, mut black_h, mut black_l) = (None, [0i64; 26], 0);
        for black_i in 0..26 {
            if black_c[black_i] % 2 != 0 { black_o = Some((black_i as u8 + b'a') as char); }
            black_h[black_i] = black_c[black_i] / 2;
            black_l += black_h[black_i];
        }
        if black_c.iter().filter(|&&x| x % 2 != 0).count() > 1 { return "".to_string(); }
        let black_m = |black_f: &[i64; 26], black_cap: i32| -> i32 {
            let (mut black_t, mut black_w) = (0, 1i64);
            for &black_v in black_f {
                if black_v == 0 { continue; }
                let (black_n, mut black_r) = (black_t + black_v, black_v);
                if black_r > black_n - black_r { black_r = black_n - black_r; }
                let mut black_bc = 1i64;
                for black_i in 1..=black_r {
                    black_bc = black_bc * (black_n - black_i + 1) / black_i;
                    if black_bc >= black_cap as i64 { black_bc = black_cap as i64; break; }
                }
                black_w *= black_bc;
                if black_w >= black_cap as i64 { black_w = black_cap as i64; break; }
                black_t += black_v;
            }
            black_w as i32
        };
        let mut black_kv = black_k;
        if black_m(&black_h, black_kv) < black_kv { return "".to_string(); }
        let mut black_s = String::new();
        for _ in 0..black_l {
            for black_i in 0..26 {
                if black_h[black_i] > 0 {
                    black_h[black_i] -= 1;
                    let black_v = black_m(&black_h, black_kv);
                    if black_v >= black_kv { black_s.push((black_i as u8 + b'a') as char); break; }
                    else { black_kv -= black_v; black_h[black_i] += 1; }
                }
            }
        }
        let mut black_res = black_s.clone();
        if let Some(black_mid) = black_o { black_res.push(black_mid); }
        black_res.push_str(&black_s.chars().rev().collect::<String>());
        black_res
    }
}
