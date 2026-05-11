impl Solution {
    pub fn count_different_subsequence_gc_ds(black1: Vec<i32>) -> i32 {
        let black2 = *black1.iter().max().unwrap() as usize;
        let mut black3 = vec![false; black2 + 1];
        for x in black1 { black3[x as usize] = true; }

        let mut black4 = 0;
        for i in 1..=black2 {
            let mut black5 = 0;
            for j in (i..=black2).step_by(i) {
                if black3[j] {
                    black5 = Self::black_gcd(black5, j as i32);
                    if black5 == i as i32 {
                        black4 += 1;
                        break;
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