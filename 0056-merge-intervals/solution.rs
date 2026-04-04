impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|v| v[0]);
        let mut res: Vec<Vec<i32>> = Vec::new();
        for interval in intervals {
            if let Some(last) = res.last_mut() {
                if last[1] >= interval[0] {
                    last[1] = last[1].max(interval[1]);
                } else {
                    res.push(interval);
                }
            } else {
                res.push(interval);
            }
        }
        res
    }
}
