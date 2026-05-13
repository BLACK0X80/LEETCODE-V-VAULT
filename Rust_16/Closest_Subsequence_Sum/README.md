# Closest Subsequence Sum

**Difficulty:** Hard
**Tags:** Array, Two Pointers, Dynamic Programming, Bit Manipulation, Sorting, Bitmask

---

## Problem

<p>You are given an integer array <code>nums</code> and an integer <code>goal</code>.</p>

<p>You want to choose a subsequence of <code>nums</code> such that the sum of its elements is the closest possible to <code>goal</code>. That is, if the sum of the subsequence&#39;s elements is <code>sum</code>, then you want to <strong>minimize the absolute difference</strong> <code>abs(sum - goal)</code>.</p>

<p>Return <em>the <strong>minimum</strong> possible value of</em> <code>abs(sum - goal)</code>.</p>

<p>Note that a subsequence of an array is an array formed by removing some elements <strong>(possibly all or none)</strong> of the original array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [5,-7,3,5], goal = 6
<strong>Output:</strong> 0
<strong>Explanation:</strong> Choose the whole array as a subsequence, with a sum of 6.
This is equal to the goal, so the absolute difference is 0.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [7,-9,15,-2], goal = -5
<strong>Output:</strong> 1
<strong>Explanation:</strong> Choose the subsequence [7,-9,-2], with a sum of -4.
The absolute difference is abs(-4 - (-5)) = abs(1) = 1, which is the minimum.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3], goal = -7
<strong>Output:</strong> 7
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 40</code></li>
	<li><code>-10<sup>7</sup> &lt;= nums[i] &lt;= 10<sup>7</sup></code></li>
	<li><code>-10<sup>9</sup> &lt;= goal &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. The naive solution is to check all possible subsequences. This works in O(2^n).
2. Divide the array into two parts of nearly is equal size.
3. Consider all subsets of one part and make a list of all possible subset sums and sort this list.
4. Consider all subsets of the other part, and for each one, let its sum = x, do binary search to get the nearest possible value to goal - x in the first part.

## Solution

```rust
impl Solution {
    pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        let n = nums.len();
        let half = n/2;
        let mut black_left = vec![0i64];
        for i in 0..half {
            let mut tmp = black_left.clone();
            for &s in &black_left { tmp.push(s + nums[i] as i64); }
            black_left = tmp;
        }
        let mut black_right = vec![0i64];
        for i in half..n {
            let mut tmp = black_right.clone();
            for &s in &black_right { tmp.push(s + nums[i] as i64); }
            black_right = tmp;
        }
        black_right.sort_unstable();
        let mut black_ans = i64::MAX;
        for &l in &black_left {
            let t = goal as i64 - l;
            let p = black_right.partition_point(|&x| x <= t);
            if p < black_right.len() { black_ans = black_ans.min((l + black_right[p] - goal as i64).abs()); }
            if p > 0 { black_ans = black_ans.min((l + black_right[p-1] - goal as i64).abs()); }
        }
        black_ans as i32
    }
}
```