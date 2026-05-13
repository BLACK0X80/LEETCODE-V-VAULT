impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut black = 0;
        for noir in 0..nums.len() { if nums[noir] != val { nums[black] = nums[noir]; black += 1; } }
        black as i32
    }
}