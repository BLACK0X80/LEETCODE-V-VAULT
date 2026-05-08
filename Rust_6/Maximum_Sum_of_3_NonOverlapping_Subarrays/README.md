# Maximum Sum of 3 Non-Overlapping Subarrays

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Sliding Window, Prefix Sum

---

## Problem

<p>Given an integer array <code>nums</code> and an integer <code>k</code>, find three non-overlapping subarrays of length <code>k</code> with maximum sum and return them.</p>

<p>Return the result as a list of indices representing the starting position of each interval (<strong>0-indexed</strong>). If there are multiple answers, return the lexicographically smallest one.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,1,2,6,7,5,1], k = 2
<strong>Output:</strong> [0,3,5]
<strong>Explanation:</strong> Subarrays [1, 2], [2, 6], [7, 5] correspond to the starting indices [0, 3, 5].
We could have also taken [2, 1], but an answer of [1, 3, 5] would be lexicographically larger.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,1,2,1,2,1,2,1], k = 2
<strong>Output:</strong> [0,2,4]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;&nbsp;2<sup>16</sup></code></li>
	<li><code>1 &lt;= k &lt;= floor(nums.length / 3)</code></li>
</ul>



## Solution

```rust
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
```