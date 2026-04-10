impl Solution {
    pub fn max_gcd_score(black_a: Vec<i32>, black_k: i32) -> i64 {
        let black_n = black_a.len();
        let black_k = black_k as usize;
        let mut black_ans = 1i64;

        for black_i in 0..black_n {
            let mut black_x = 0;
            let mut black_mp = std::collections::BTreeMap::new();
            
            for black_j in black_i..black_n {
                let black_val = black_a[black_j];
                black_x = if black_x == 0 { black_val } else { Self::black_gcd(black_x, black_val) };
                
                let black_cnt = black_val.trailing_zeros() as i32;
                *black_mp.entry(black_cnt).or_insert(0) += 1;
                
                let (&black_mi, &black_cnn) = black_mp.iter().next().unwrap();
                let black_len = (black_j - black_i + 1) as i64;
                let black_xx = black_x as i64;
                
                if black_k >= black_cnn {
                    black_ans = black_ans.max(black_xx * 2 * black_len);
                } else {
                    black_ans = black_ans.max(black_xx * black_len);
                }
                
                if (black_n - black_i) as i64 * (black_xx * 2) <= black_ans {
                    break;
                }
            }
        }
        black_ans
    }

    fn black_gcd(mut black_u: i32, mut black_v: i32) -> i32 {
        while black_v != 0 {
            black_u %= black_v;
            std::mem::swap(&mut black_u, &mut black_v);
        }
        black_u
    }
}
