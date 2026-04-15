# Apply Operations to Maximize Frequency Score

**Difficulty:** Hard
**Tags:** Array, Binary Search, Sliding Window, Sorting, Prefix Sum

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code> and an integer <code>k</code>.</p>

<p>You can perform the following operation on the array <strong>at most</strong> <code>k</code> times:</p>

<ul>
	<li>Choose any index <code>i</code> from the array and <strong>increase</strong> or <strong>decrease</strong> <code>nums[i]</code> by <code>1</code>.</li>
</ul>

<p>The score of the final array is the <strong>frequency</strong> of the most frequent element in the array.</p>

<p>Return <em>the <strong>maximum</strong> score you can achieve</em>.</p>

<p>The frequency of an element is the number of occurences of that element in the array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,6,4], k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can do the following operations on the array:
- Choose i = 0, and increase the value of nums[0] by 1. The resulting array is [2,2,6,4].
- Choose i = 3, and decrease the value of nums[3] by 1. The resulting array is [2,2,6,3].
- Choose i = 3, and decrease the value of nums[3] by 1. The resulting array is [2,2,6,2].
The element 2 is the most frequent in the final array so our score is 3.
It can be shown that we cannot achieve a better score.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,4,4,2,4], k = 0
<strong>Output:</strong> 3
<strong>Explanation:</strong> We cannot apply any operations so our score will be the frequency of the most frequent element in the original array, which is 3.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= k &lt;= 10<sup>14</sup></code></li>
</ul>


## Hints

1. If you sort the original array, it is optimal to apply the operations on one subarray such that all the elements of that subarray become equal.
2. You can use binary search to find the longest subarray where we can make the elements equal in at most <code>k</code> operations.

## Solution

```rust
impl Solution {
    pub fn max_frequency_score(mut black_nums: Vec<i32>, black_k: i64) -> i32 {
        black_nums.sort_unstable();
        let black_n = black_nums.len();
        let mut black_pre = vec![0i64; black_n + 1];
        for i in 0..black_n { black_pre[i+1] = black_pre[i] + black_nums[i] as i64; }

        let mut black_l = 0;
        let mut black_ans = 0;
        let bravexuneth = &black_nums;

        for black_r in 0..black_n {
            while black_l <= black_r {
                let black_mid = (black_l + black_r) / 2;
                let black_target = bravexuneth[black_mid] as i64;
                
                let black_left_count = (black_mid - black_l) as i64;
                let black_right_count = (black_r - black_mid) as i64;
                
                let black_cost_left = black_left_count * black_target - (black_pre[black_mid] - black_pre[black_l]);
                let black_cost_right = (black_pre[black_r + 1] - black_pre[black_mid + 1]) - black_right_count * black_target;
                
                if black_cost_left + black_cost_right <= black_k { break; }
                black_l += 1;
            }
            black_ans = black_ans.max(black_r - black_l + 1);
        }
        black_ans as i32
    }
}
```