use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let black: HashSet<i32> = nums.into_iter().collect();
        let mut ans = 0;
        for &num in &black {
            if !black.contains(&(num - 1)) {
                let mut curr = num;
                let mut len = 1;
                while black.contains(&(curr + 1)) {
                    curr += 1;
                    len += 1;
                }
                if len > ans { ans = len; }
            }
        }
        ans
    }
}