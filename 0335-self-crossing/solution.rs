impl Solution {
    pub fn is_self_crossing(d: Vec<i32>) -> bool {
        let n = d.len();
        for i in 3..n {
            if d[i] >= d[i-2] && d[i-1] <= d[i-3] {
                return true;
            }
            if i >= 4 && d[i-1] == d[i-3] && d[i] + d[i-4] >= d[i-2] {
                return true;
            }
            if i >= 5 && d[i-2] > d[i-4] && d[i] + d[i-4] >= d[i-2] && d[i-1] < d[i-3] && d[i-1] + d[i-5] >= d[i-3] {
                return true;
            }
        }
        false
    }
}
