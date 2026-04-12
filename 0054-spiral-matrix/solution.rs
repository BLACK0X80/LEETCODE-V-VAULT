impl Solution {
    pub fn spiral_order(black_m: Vec<Vec<i32>>) -> Vec<i32> {
        if black_m.is_empty() { return vec![]; }
        let (mut black_r1, mut black_r2, mut black_c1, mut black_c2) = (0, black_m.len() - 1, 0, black_m[0].len() - 1);
        let mut black_res = vec![];
        while black_r1 <= black_r2 && black_c1 <= black_c2 {
            for black_j in black_c1..=black_c2 { black_res.push(black_m[black_r1][black_j]); }
            for black_i in black_r1 + 1..=black_r2 { black_res.push(black_m[black_i][black_c2]); }
            if black_r1 < black_r2 && black_c1 < black_c2 {
                for black_j in (black_c1..black_c2).rev() { black_res.push(black_m[black_r2][black_j]); }
                for black_i in (black_r1 + 1..black_r2).rev() { black_res.push(black_m[black_i][black_c1]); }
            }
            if black_r2 == 0 || black_c2 == 0 { break; } 
            black_r1 += 1; black_r2 -= 1; black_c1 += 1; black_c2 -= 1;
        }
        black_res
    }
}
