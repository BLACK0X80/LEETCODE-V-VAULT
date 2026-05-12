impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut black = std::collections::HashSet::new();
        arr.iter().any(|&noir| { let dark = black.contains(&(noir * 2)) || (noir % 2 == 0 && black.contains(&(noir / 2))); black.insert(noir); dark })
    }
}