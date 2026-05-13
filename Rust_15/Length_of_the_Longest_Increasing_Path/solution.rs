impl Solution {
    pub fn max_path_length(mut black_coords: Vec<Vec<i32>>, black_k: i32) -> i32 {
        let black_target = black_coords[black_k as usize].clone();
        
        let mut black_calc = |black_pts: Vec<&Vec<i32>>| -> i32 {
            let mut black_pts = black_pts;
            black_pts.sort_by(|black_a, black_b| black_a[0].cmp(&black_b[0]).then(black_b[1].cmp(&black_a[1])));
            let mut black_tails = Vec::new();
            for black_p in black_pts {
                let black_y = black_p[1];
                let black_pos = black_tails.binary_search(&black_y).unwrap_or_else(|black_e| black_e);
                if black_pos < black_tails.len() { black_tails[black_pos] = black_y; }
                else { black_tails.push(black_y); }
            }
            black_tails.len() as i32
        };

        let black_before: Vec<&Vec<i32>> = black_coords.iter().filter(|black_p| black_p[0] < black_target[0] && black_p[1] < black_target[1]).collect();
        let black_after: Vec<&Vec<i32>> = black_coords.iter().filter(|black_p| black_p[0] > black_target[0] && black_p[1] > black_target[1]).collect();

        black_calc(black_before) + 1 + black_calc(black_after)
    }
}