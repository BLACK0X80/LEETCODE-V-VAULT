impl Solution {
    pub fn insert(black_ints: Vec<Vec<i32>>, mut black_new: Vec<i32>) -> Vec<Vec<i32>> {
        let mut black_res = vec![];
        let mut black_i = 0;
        while black_i < black_ints.len() && black_ints[black_i][1] < black_new[0] {
            black_res.push(black_ints[black_i].clone());
            black_i += 1;
        }
        while black_i < black_ints.len() && black_ints[black_i][0] <= black_new[1] {
            black_new[0] = black_new[0].min(black_ints[black_i][0]);
            black_new[1] = black_new[1].max(black_ints[black_i][1]);
            black_i += 1;
        }
        black_res.push(black_new);
        black_res.extend(black_ints[black_i..].iter().cloned());
        black_res
    }
}