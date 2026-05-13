use std::collections::HashMap;

impl Solution {
    pub fn beautiful_substrings(black_s: String, black_k: i32) -> i64 {
        let black_minimal = {
            let mut black_temp_k = black_k;
            let mut black_min_v = 1;
            let mut i = 2;
            while i * i <= black_temp_k {
                if black_temp_k % i == 0 {
                    let mut black_p = 0;
                    while black_temp_k % i == 0 {
                        black_temp_k /= i;
                        black_p += 1;
                    }
                    black_min_v *= i.pow((black_p + 1) / 2);
                }
                i += 1;
            }
            if black_temp_k > 1 { black_min_v *= black_temp_k; }
            black_min_v * 2
        };

        let mut black_seen = vec![HashMap::new(); black_minimal as usize];
        black_seen[0].insert(0, 1i64);

        let (mut black_psum, mut black_res) = (0i32, 0i64);
        let black_bytes = black_s.as_bytes();

        for i in 0..black_bytes.len() {
            let black_c = black_bytes[i];
            black_psum += if b"aeiou".contains(&black_c) { 1 } else { -1 };

            let black_group = (i + 1) % black_minimal as usize;
            if let Some(&black_count) = black_seen[black_group].get(&black_psum) {
                black_res += black_count;
            }
            *black_seen[black_group].entry(black_psum).or_insert(0) += 1;
        }

        black_res
    }
}