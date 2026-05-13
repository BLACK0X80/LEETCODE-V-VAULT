impl Solution {
    pub fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
        let black1 = board.len();
        let black2 = board[0].len();
        let mut black3 = vec![vec![]; black1];

        for black4 in 0..black1 {
            let mut black5: Vec<(i64, usize)> = board[black4].iter().enumerate()
                .map(|(i, &v)| (v as i64, i)).collect();
            black5.sort_by(|a, b| b.0.cmp(&a.0));
            black3[black4] = black5[..3].to_vec();
        }

        let mut black6 = i64::MIN;
        for black7 in 0..black1 {
            for black8 in black7 + 1..black1 {
                for black9 in black8 + 1..black1 {
                    for black10 in &black3[black7] {
                        for black11 in &black3[black8] {
                            if black11.1 == black10.1 { continue; }
                            for black12 in &black3[black9] {
                                if black12.1 == black10.1 || black12.1 == black11.1 { continue; }
                                black6 = black6.max(black10.0 + black11.0 + black12.0);
                            }
                        }
                    }
                }
            }
        }
        black6
    }
}