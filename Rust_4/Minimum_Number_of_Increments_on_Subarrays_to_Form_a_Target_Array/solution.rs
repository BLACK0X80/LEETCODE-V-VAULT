impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        target.windows(2).fold(target[0], |acc, w| {
            acc + (w[1] - w[0]).max(0)
        })
    }
}