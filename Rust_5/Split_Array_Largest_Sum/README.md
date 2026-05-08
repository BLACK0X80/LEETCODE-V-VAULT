# Split Array Largest Sum

**Difficulty:** Hard
**Tags:** Array, Binary Search, Dynamic Programming, Greedy, Prefix Sum

---

## Problem

<p>Given an integer array <code>nums</code> and an integer <code>k</code>, split <code>nums</code> into <code>k</code> non-empty subarrays such that the largest sum of any subarray is <strong>minimized</strong>.</p>

<p>Return <em>the minimized largest sum of the split</em>.</p>

<p>A <strong>subarray</strong> is a contiguous part of the array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [7,2,5,10,8], k = 2
<strong>Output:</strong> 18
<strong>Explanation:</strong> There are four ways to split nums into two subarrays.
The best way is to split it into [7,2,5] and [10,8], where the largest sum among the two subarrays is only 18.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4,5], k = 2
<strong>Output:</strong> 9
<strong>Explanation:</strong> There are four ways to split nums into two subarrays.
The best way is to split it into [1,2,3] and [4,5], where the largest sum among the two subarrays is only 9.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
	<li><code>1 &lt;= k &lt;= min(50, nums.length)</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut black_left = 0;
        let mut black_right = 0;
        for &black_num in &nums {
            black_left = black_left.max(black_num);
            black_right += black_num as i64;
        }
        let mut black_ans = black_right;
        let mut black_l = black_left as i64;
        let mut black_r = black_right;
        while black_l <= black_r {
            let black_mid = black_l + (black_r - black_l) / 2;
            let mut black_splits = 1;
            let mut black_current_sum = 0;
            for &black_val in &nums {
                if black_current_sum + black_val as i64 > black_mid {
                    black_splits += 1;
                    black_current_sum = black_val as i64;
                } else {
                    black_current_sum += black_val as i64;
                }
            }
            if black_splits <= k {
                black_ans = black_mid;
                black_r = black_mid - 1;
            } else {
                black_l = black_mid + 1;
            }
        }
        black_ans as i32
    }
}
```