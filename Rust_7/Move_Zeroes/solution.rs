impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut black = 0;
        for noir in 0..nums.len() { if nums[noir] != 0 { nums.swap(black, noir); black += 1; } }
    }
}