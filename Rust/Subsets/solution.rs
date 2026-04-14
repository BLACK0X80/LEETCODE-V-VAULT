impl Solution {
    pub fn subsets(black_nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut black_res = vec![vec![]];
        for black_x in black_nums {
            let black_len = black_res.len();
            for black_i in 0..black_len {
                let mut black_sub = black_res[black_i].clone();
                black_sub.push(black_x);
                black_res.push(black_sub);
            }
        }
        black_res
    }
}