impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        const M: u64 = 1_000_000_007;
        let (mut a, mut e, mut i, mut o, mut u) = (1u64, 1, 1, 1, 1);
        for _ in 1..n {
            (a, e, i, o, u) = (
                (e + i + u) % M,
                (a + i) % M,
                (e + o) % M,
                i % M,
                (i + o) % M,
            );
        }
        ((a + e + i + o + u) % M) as i32
    }
}
