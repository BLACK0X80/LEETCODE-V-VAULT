impl Solution {
    pub fn solve_queries(black: Vec<i32>, black_black: Vec<i32>) -> Vec<i32> {
        { let mut black_m = std::collections::HashMap::new(); for (black_i, &black_x) in black.iter().enumerate() { black_m.entry(black_x).or_insert(vec![]).push(black_i as i32); } black_black.iter().map(|&black_b| { let black_p = &black_m[&black[black_b as usize]]; if black_p.len() < 2 { -1 } else { let black_n = black.len() as i32; let black_idx = black_p.binary_search(&(black_b as i32)).unwrap(); let black_l = black_p[(black_idx + black_p.len() - 1) % black_p.len()]; let black_r = black_p[(black_idx + 1) % black_p.len()]; std::cmp::min((black_b - black_l + black_n) % black_n, (black_r - black_b + black_n) % black_n) } }).collect() }
    }
}