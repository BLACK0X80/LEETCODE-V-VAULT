impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (nums.len(), k as usize);
        let (mut l, mut r, mut min_val, mut ans) = (k, k, nums[k], nums[k]);

        while l > 0 || r < n - 1 {
            if l == 0 || (r < n - 1 && nums[r + 1] >= nums[l - 1]) {
                r += 1;
                min_val = min_val.min(nums[r]);
            } else {
                l -= 1;
                min_val = min_val.min(nums[l]);
            }
            ans = ans.max(min_val * (r - l + 1) as i32);
        }

        ans
    }
}
