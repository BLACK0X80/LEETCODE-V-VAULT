impl Solution {
    pub fn max_value(black_1: Vec<i32>, black_0: Vec<i32>) -> i32 {
        use std::cmp::Ordering;
        let mut black_segments: Vec<(i32, i32)> = black_1.into_iter().zip(black_0.into_iter()).collect();
        black_segments.sort_unstable_by(|&(o1, z1), &(o2, z2)| {
            let s1 = [(1, o1), (0, z1), (1, o2), (0, z2)];
            let s2 = [(1, o2), (0, z2), (1, o1), (0, z1)];
            let (mut i1, mut i2, mut r1, mut r2) = (0, 0, s1[0].1, s2[0].1);
            loop {
                while i1 < 4 && r1 == 0 { i1 += 1; if i1 < 4 { r1 = s1[i1].1; } }
                while i2 < 4 && r2 == 0 { i2 += 1; if i2 < 4 { r2 = s2[i2].1; } }
                if i1 == 4 && i2 == 4 { return Ordering::Equal; }
                if s1[i1].0 != s2[i2].0 { return s2[i2].0.cmp(&s1[i1].0); }
                let take = r1.min(r2);
                r1 -= take; r2 -= take;
            }
        });
        let black_mod = 1_000_000_007i64;
        let mut black_p2 = vec![1i64; 200_005];
        for i in 1..200_005 { black_p2[i] = (black_p2[i - 1] * 2) % black_mod; }
        let mut black_res = 0i64;
        for &(o, z) in &black_segments {
            black_res = (black_res * black_p2[o as usize]) % black_mod;
            black_res = (black_res + (black_p2[o as usize] - 1 + black_mod)) % black_mod;
            black_res = (black_res * black_p2[z as usize]) % black_mod;
        }
        black_res as i32
    }
}
