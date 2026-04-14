impl Solution {
    pub fn next_permutation(black_nums: &mut Vec<i32>) {
        if let Some(black_i) = black_nums.windows(2).rposition(|w| w[0] < w[1]) {
            let black_j = black_nums.iter().rposition(|&x| x > black_nums[black_i]).unwrap();
            black_nums.swap(black_i, black_j);
            black_nums[black_i + 1..].reverse();
        } else { black_nums.reverse(); }
    }
}