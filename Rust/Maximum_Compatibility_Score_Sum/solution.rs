impl Solution {
    pub fn max_compatibility_sum(black_s: Vec<Vec<i32>>, black_m: Vec<Vec<i32>>) -> i32 {
        let black_n = black_s.len(); let mut black_p = vec![0; black_n]; for i in 0..black_n { black_p[i] = i; }
        let mut black_max = 0; loop { let black_score: i32 = (0..black_n).map(|i| black_s[i].iter().zip(&black_m[black_p[i]]).filter(|&(a, b)| a == b).count() as i32).sum(); black_max = black_max.max(black_score); let mut i = black_n - 1; while i > 0 && black_p[i-1] >= black_p[i] { i -= 1; } if i == 0 { break; } let mut j = black_n - 1; while black_p[j] <= black_p[i-1] { j -= 1; } black_p.swap(i-1, j); black_p[i..].reverse(); }
        black_max
    }
}