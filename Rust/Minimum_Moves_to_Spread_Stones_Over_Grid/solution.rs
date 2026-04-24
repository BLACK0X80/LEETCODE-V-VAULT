impl Solution {
    pub fn minimum_moves(black_grid: Vec<Vec<i32>>) -> i32 {
        let (mut black_src, mut black_dst) = (vec![], vec![]);
        for black_r in 0..3 { for black_c in 0..3 { if black_grid[black_r][black_c] == 0 { black_dst.push((black_r, black_c)); } else if black_grid[black_r][black_c] > 1 { for _ in 0..black_grid[black_r][black_c]-1 { black_src.push((black_r, black_c)); } } } }
        let mut black_min = i32::MAX; fn black_perm(mut b_s: Vec<(usize,usize)>, b_d: &Vec<(usize,usize)>, b_idx: usize, b_m: &mut i32) { if b_idx == b_s.len() { *b_m = (*b_m).min(b_s.iter().zip(b_d).map(|(s, d)| (s.0 as i32 - d.0 as i32).abs() + (s.1 as i32 - d.1 as i32).abs()).sum()); return; } for i in b_idx..b_s.len() { b_s.swap(b_idx, i); black_perm(b_s.clone(), b_d, b_idx + 1, b_m); b_s.swap(b_idx, i); } }
        black_perm(black_src, &black_dst, 0, &mut black_min); black_min
    }
}