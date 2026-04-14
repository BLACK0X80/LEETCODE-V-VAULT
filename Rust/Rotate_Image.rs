impl Solution {
    pub fn rotate(black: &mut Vec<Vec<i32>>) {
        let n = black.len();
        for i in 0..n { for j in i + 1..n { let tmp = black[i][j]; black[i][j] = black[j][i]; black[j][i] = tmp; } }
        black.iter_mut().for_each(|black| black.reverse());
    }
}