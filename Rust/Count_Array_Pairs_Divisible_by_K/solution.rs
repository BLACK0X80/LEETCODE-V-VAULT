use std::collections::HashMap;
impl Solution {
    pub fn count_pairs(black1: Vec<i32>, black2: i32) -> i64 {
        let mut black3 = HashMap::new();
        for &x in &black1 {
            let g = Self::black_gcd(x, black2);
            *black3.entry(g).or_insert(0i64) += 1;
        }

        let mut black4 = 0i64;
        let black5: Vec<_> = black3.keys().cloned().collect();
        for i in 0..black5.len() {
            for j in i..black5.len() {
                let g1 = black5[i];
                let g2 = black5[j];
                if (g1 as i64 * g2 as i64) % black2 as i64 == 0 {
                    if i == j {
                        black4 += black3[&g1] * (black3[&g1] - 1) / 2;
                    } else {
                        black4 += black3[&g1] * black3[&g2];
                    }
                }
            }
        }
        black4
    }

    fn black_gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 { a %= b; std::mem::swap(&mut a, &mut b); }
        a
    }
}