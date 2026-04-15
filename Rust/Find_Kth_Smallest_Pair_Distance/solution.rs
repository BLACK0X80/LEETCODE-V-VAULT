impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let n = nums.len();
        let (mut lo, mut hi) = (0, nums[n-1] - nums[0]);
        while lo < hi {
            let mid = (lo + hi) / 2;
            let mut cnt = 0;
            let mut l = 0;
            for r in 0..n { while nums[r] - nums[l] > mid { l += 1; } cnt += r - l; }
            if cnt >= k as usize { hi = mid; } else { lo = mid + 1; }
        }
        lo
    }
}