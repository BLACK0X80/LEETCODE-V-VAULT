impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            let m = l + (r - l) / 2;
            if nums[m] == target { return m as i32; }
            if nums[l] <= nums[m] {
                if nums[l] <= target && target < nums[m] { r = m - 1; } else { l = m + 1; }
            } else {
                if nums[m] < target && target <= nums[r] { l = m + 1; } else { r = m - 1; }
            }
        }
        -1
    }
}
