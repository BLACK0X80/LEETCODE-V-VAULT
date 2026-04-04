impl Solution {
    pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
        let mut res = vec![];
        let mut ti = 0;
        let mut cur = 1;
        while ti < target.len() {
            res.push("Push".to_string());
            if cur == target[ti] { ti += 1; }
            else { res.push("Pop".to_string()); }
            cur += 1;
        }
        res
    }
}
