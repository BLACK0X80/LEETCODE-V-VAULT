impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut win = vec![0i32; n - k + 1];
        let mut sum = 0;
        for i in 0..k { sum += nums[i]; }
        win[0] = sum;
        for i in 1..=n-k {
            sum += nums[i+k-1] - nums[i-1];
            win[i] = sum;
        }

        let m = win.len();
        let mut left = vec![0usize; m];
        let mut best = 0;
        for i in 0..m {
            if win[i] > win[best] { best = i; }
            left[i] = best;
        }

        let mut right = vec![0usize; m];
        best = m - 1;
        for i in (0..m).rev() {
            if win[i] >= win[best] { best = i; }
            right[i] = best;
        }

        let mut res = vec![-1i32; 3];
        let mut max_sum = 0;
        for j in k..m-k {
            let (l, r) = (left[j-k], right[j+k]);
            let s = win[l] + win[j] + win[r];
            if s > max_sum {
                max_sum = s;
                res = vec![l as i32, j as i32, r as i32];
            }
        }

        res
    }
}
