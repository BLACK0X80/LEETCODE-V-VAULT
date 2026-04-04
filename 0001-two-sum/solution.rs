use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            let c = target - n;
            if let Some(&j) = m.get(&c) {
                return vec![j as i32, i as i32];
            }
            m.insert(n, i);
        }
        vec![]
    }
}
