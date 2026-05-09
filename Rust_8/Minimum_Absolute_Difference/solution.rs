impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let mut res = Vec::new();
        let mut min = i32::MAX;
        for w in arr.windows(2) {
            let d = w[1] - w[0];
            if d < min {
                min = d;
                res.clear();
                res.push(vec![w[0], w[1]]);
            } else if d == min {
                res.push(vec![w[0], w[1]]);
            }
        }
        res
    }
}