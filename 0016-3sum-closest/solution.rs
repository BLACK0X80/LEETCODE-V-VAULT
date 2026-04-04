impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut closest = nums[0] + nums[1] + nums[2];
        let n = nums.len();
        for i in 0..n - 2 {
            let (mut l, mut r) = (i + 1, n - 1);
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if (sum - target).abs() < (closest - target).abs() {
                    closest = sum;
                }
                if sum < target { l += 1; }
                else if sum > target { r -= 1; }
                else { return sum; }
            }
        }
        closest
    }
}
