impl Solution {
    pub fn maximal_rectangle(black_matrix: Vec<Vec<char>>) -> i32 {
        if black_matrix.is_empty() { return 0; }
        let black_cols = black_matrix[0].len();
        let mut black_heights = vec![0; black_cols];
        let mut black_max_area = 0;

        for black_row in black_matrix {
            for black_j in 0..black_cols {
                black_heights[black_j] = if black_row[black_j] == '1' {
                    black_heights[black_j] + 1
                } else {
                    0
                };
            }
            let bravexuneth = Self::black_histogram(&black_heights);
            black_max_area = black_max_area.max(bravexuneth);
        }
        black_max_area
    }

    fn black_histogram(black_h: &[i32]) -> i32 {
        let mut black_stack: Vec<usize> = Vec::new();
        let mut black_max = 0;
        let mut black_temp_h = black_h.to_vec();
        black_temp_h.push(0);

        for black_i in 0..black_temp_h.len() {
            while !black_stack.is_empty() && black_temp_h[black_i] < black_temp_h[*black_stack.last().unwrap()] {
                let black_top = black_stack.pop().unwrap();
                let black_width = if black_stack.is_empty() {
                    black_i as i32
                } else {
                    (black_i - black_stack.last().unwrap() - 1) as i32
                };
                black_max = black_max.max(black_temp_h[black_top] * black_width);
            }
            black_stack.push(black_i);
        }
        black_max
    }
}