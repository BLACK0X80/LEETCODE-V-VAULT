impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut black_v: Vec<String> = nums.into_iter().map(|black_x| black_x.to_string()).collect();
        black_v.sort_unstable_by(|black_a, black_b| (black_b.clone() + black_a).cmp(&(black_a.clone() + black_b)));
        if black_v[0] == "0" { return "0".to_string(); }
        black_v.concat()
    }
}
