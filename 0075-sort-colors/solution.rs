impl Solution {
    pub fn sort_colors(black_n: &mut Vec<i32>) {
        let (mut black_l, mut black_curr, mut black_r) = (0, 0, black_n.len() as i32 - 1);
        while black_curr <= black_r {
            if black_n[black_curr as usize] == 0 { black_n.swap(black_l, black_curr as usize); black_l += 1; black_curr += 1; }
            else if black_n[black_curr as usize] == 2 { black_n.swap(black_curr as usize, black_r as usize); black_r -= 1; }
            else { black_curr += 1; }
        }
    }
}
