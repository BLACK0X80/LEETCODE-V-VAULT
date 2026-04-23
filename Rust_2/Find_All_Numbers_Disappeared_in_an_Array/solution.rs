impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let idx = nums[i].abs() as usize - 1;
            if nums[idx] > 0 { nums[idx] = -nums[idx]; }
        }
        (1..=nums.len()).filter(|&i| nums[i-1] > 0).map(|i| i as i32).collect()
    }
}