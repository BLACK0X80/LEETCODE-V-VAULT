struct NumMatrix { black_p: Vec<Vec<i32>> }
impl NumMatrix {
    fn new(black_m: Vec<Vec<i32>>) -> Self {
        let (r, c) = (black_m.len(), black_m[0].len());
        let mut p = vec![vec![0; c + 1]; r + 1];
        for i in 0..r { for j in 0..c { p[i+1][j+1] = black_m[i][j] + p[i][j+1] + p[i+1][j] - p[i][j]; } }
        Self { black_p: p }
    }
    fn sum_region(&self, r1: i32, c1: i32, r2: i32, c2: i32) -> i32 {
        let (r1, c1, r2, c2) = (r1 as usize, c1 as usize, r2 as usize, c2 as usize);
        self.black_p[r2+1][c2+1] - self.black_p[r1][c2+1] - self.black_p[r2+1][c1] + self.black_p[r1][c1]
    }
}
