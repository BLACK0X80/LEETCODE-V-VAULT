struct SubrectangleQueries {
    black: Vec<Vec<i32>>,
    updates: Vec<(i32, i32, i32, i32, i32)>,
}

impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        SubrectangleQueries { black: rectangle, updates: vec![] }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        self.updates.push((row1, col1, row2, col2, new_value));
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        for &(r1, c1, r2, c2, v) in self.updates.iter().rev() {
            if r1 <= row && row <= r2 && c1 <= col && col <= c2 { return v; }
        }
        self.black[row as usize][col as usize]
    }
}