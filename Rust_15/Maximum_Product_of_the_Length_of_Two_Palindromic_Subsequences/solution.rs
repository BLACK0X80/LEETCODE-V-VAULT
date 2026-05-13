impl Solution {
    pub fn max_product(black_s: String) -> i32 {
        let (black_n, mut black_p, mut black_res) = (black_s.len(), std::collections::HashMap::new(), 0);
        for black_m in 1..(1 << black_n) { let black_sub: String = black_s.chars().enumerate().filter(|&(i, _)| (black_m >> i) & 1 == 1).map(|(_, c)| c).collect(); if black_sub == black_sub.chars().rev().collect::<String>() { black_p.insert(black_m, black_sub.len() as i32); } }
        for (black_m1, black_l1) in &black_p { for (black_m2, black_l2) in &black_p { if black_m1 & black_m2 == 0 { black_res = black_res.max(black_l1 * black_l2); } } }
        black_res
    }
}