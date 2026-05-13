use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut black = HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            if let Some(&last) = black.get(&n) {
                if i as i32 - last <= k { return true; }
            }
            black.insert(n, i as i32);
        }
        false
    }
}