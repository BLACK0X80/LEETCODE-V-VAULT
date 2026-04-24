impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut black = 1;
        for noir in 1..nums.len() { if nums[noir] != nums[noir - 1] { nums[black] = nums[noir]; black += 1; } }
        black as i32
    }
}