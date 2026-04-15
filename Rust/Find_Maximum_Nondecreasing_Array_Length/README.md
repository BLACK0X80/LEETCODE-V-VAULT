# Find Maximum Non-decreasing Array Length

**Difficulty:** Hard
**Tags:** Array, Binary Search, Dynamic Programming, Stack, Queue, Monotonic Stack, Monotonic Queue

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code>.</p>

<p>You can perform any number of operations, where each operation involves selecting a <strong>subarray</strong> of the array and replacing it with the <strong>sum</strong> of its elements. For example, if the given array is <code>[1,3,5,6]</code> and you select subarray <code>[3,5]</code> the array will convert to <code>[1,8,6]</code>.</p>

<p>Return <em>the </em><strong><em>maximum</em></strong><em> length of a </em><strong><em>non-decreasing</em></strong><em> array that can be made after applying operations.</em></p>

<p>A <strong>subarray</strong> is a contiguous <strong>non-empty</strong> sequence of elements within an array.</p>

<p>&nbsp;</p>
<p><strong>Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [5,2,2]
<strong>Output:</strong> 1
<strong>Explanation:</strong> This array with length 3 is not non-decreasing.
We have two ways to make the array length two.
First, choosing subarray [2,2] converts the array to [5,4].
Second, choosing subarray [5,2] converts the array to [7,2].
In these two ways the array is not non-decreasing.
And if we choose subarray [5,2,2] and replace it with [9] it becomes non-decreasing. 
So the answer is 1.
</pre>

<p><strong>Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The array is non-decreasing. So the answer is 4.
</pre>

<p><strong>Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [4,3,2,6]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Replacing [3,2] with [5] converts the given array to [4,5,6] that is non-decreasing.
Because the given array is not non-decreasing, the maximum<!-- notionvc: 3447a505-d1ee-4411-8cae-e52162f53a55 --> possible answer is 3.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Let <code>dp[i]</code> be the maximum number of elements in the increasing sequence after processing the first <code>i</code> elements of the original array.
2. We have <code>dp[0] = 0</code>. <code>dp[i + 1] >= dp[i]</code> (since if we have the solution for the first <code>i</code> elements, we can always merge the last one of the first <code>i + 1</code> elements which is <code>nums[i]</code> into the solution of the first <code>i</code> elements.
3. For <code>i > 0</code>, we want to <code>dp[i] = max(dp[j] + 1)</code> where <code>sum(nums[i - 1] + nums[i - 2] +… + nums[j]) >= v[j]</code> and <code>v[j]</code> is the last element of the solution ending with <code>nums[j - 1]</code>.

## Solution

```rust
impl Solution {
    pub fn find_maximum_length(black_nums: Vec<i32>) -> i32 {
        let black_n = black_nums.len();
        let mut black_pre = vec![0i64; black_n + 1];
        for i in 0..black_n { black_pre[i+1] = black_pre[i] + black_nums[i] as i64; }

        let mut black_dp = vec![0; black_n + 1];
        let mut black_last = vec![0i64; black_n + 1];
        let mut black_q = std::collections::VecDeque::new();
        black_q.push_back(0);
        
        let bravexuneth = &black_pre;

        for black_i in 1..=black_n {
            while black_q.len() > 1 && bravexuneth[black_q[1]] + black_last[black_q[1]] <= bravexuneth[black_i] {
                black_q.pop_front();
            }
            
            let black_j = black_q[0];
            black_dp[black_i] = black_dp[black_j] + 1;
            black_last[black_i] = bravexuneth[black_i] - bravexuneth[black_j];
            
            while !black_q.is_empty() {
                let black_prev = *black_q.back().unwrap();
                if bravexuneth[black_prev] + black_last[black_prev] >= bravexuneth[black_i] + black_last[black_i] {
                    black_q.pop_back();
                } else { break; }
            }
            black_q.push_back(black_i);
        }
        black_dp[black_n]
    }
}
```