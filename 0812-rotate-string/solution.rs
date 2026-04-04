impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        s.len() == goal.len() && format!("{}{}", s, s).contains(&goal as &str)
    }
}
