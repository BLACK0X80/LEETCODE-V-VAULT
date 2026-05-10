# Minimum Total Space Wasted With K Resizing Operations

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Prefix Sum

---

## Problem

<p>You are currently designing a dynamic array. You are given a <strong>0-indexed</strong> integer array <code>nums</code>, where <code>nums[i]</code> is the number of elements that will be in the array at time <code>i</code>. In addition, you are given an integer <code>k</code>, the <strong>maximum</strong> number of times you can <strong>resize</strong> the array (to<strong> any</strong> size).</p>

<p>The size of the array at time <code>t</code>, <code>size<sub>t</sub></code>, must be at least <code>nums[t]</code> because there needs to be enough space in the array to hold all the elements. The <strong>space wasted</strong> at&nbsp;time <code>t</code> is defined as <code>size<sub>t</sub> - nums[t]</code>, and the <strong>total</strong> space wasted is the <strong>sum</strong> of the space wasted across every time <code>t</code> where <code>0 &lt;= t &lt; nums.length</code>.</p>

<p>Return <em>the <strong>minimum</strong> <strong>total space wasted</strong> if you can resize the array at most</em> <code>k</code> <em>times</em>.</p>

<p><strong>Note:</strong> The array can have <strong>any size</strong> at the start and does<strong> not </strong>count towards the number of resizing operations.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [10,20], k = 0
<strong>Output:</strong> 10
<strong>Explanation:</strong> size = [20,20].
We can set the initial size to be 20.
The total wasted space is (20 - 10) + (20 - 20) = 10.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [10,20,30], k = 1
<strong>Output:</strong> 10
<strong>Explanation:</strong> size = [20,20,30].
We can set the initial size to be 20 and resize to 30 at time 2. 
The total wasted space is (20 - 10) + (20 - 20) + (30 - 30) = 10.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [10,20,15,30,20], k = 2
<strong>Output:</strong> 15
<strong>Explanation:</strong> size = [10,20,20,30,30].
We can set the initial size to 10, resize to 20 at time 1, and resize to 30 at time 3.
The total wasted space is (10 - 10) + (20 - 20) + (20 - 15) + (30 - 30) + (30 - 20) = 15.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 200</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
	<li><code>0 &lt;= k &lt;= nums.length - 1</code></li>
</ul>


## Hints

1. Given a range, how can you find the minimum waste if you can't perform any resize operations?
2. Can we build our solution using dynamic programming using the current index and the number of resizing operations performed as the states?

## Solution

```rust
impl Solution { pub fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 { let black_n = nums.len(); let mut black_waste = vec![vec![0; black_n]; black_n]; for i in 0..black_n { let (mut black_max, mut black_sum) = (0, 0); for j in i..black_n { black_max = black_max.max(nums[j]); black_sum += nums[j]; black_waste[i][j] = black_max * (j - i + 1) as i32 - black_sum; } } let mut black_dp = vec![vec![i32::MAX / 2; k as usize + 2]; black_n + 1]; black_dp[0][0] = 0; for i in 1..=black_n { for j in 1..=k as usize + 1 { for p in 0..i { black_dp[i][j] = black_dp[i][j].min(black_dp[p][j-1] + black_waste[p][i-1]); } } } black_dp[black_n][k as usize + 1] } }
```