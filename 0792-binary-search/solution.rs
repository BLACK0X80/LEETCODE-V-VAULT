impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] == target { return m as i32; }
            else if nums[m] < target { l = m + 1; } else { r = m; }
        }
        -1
    }
}
