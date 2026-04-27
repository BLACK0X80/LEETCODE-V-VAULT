# Greatest Sum Divisible by Three

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Greedy, Sorting

---

## Problem

<p>Given an integer array <code>nums</code>, return <em>the <strong>maximum possible sum </strong>of elements of the array such that it is divisible by three</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,6,5,1,8]
<strong>Output:</strong> 18
<strong>Explanation:</strong> Pick numbers 3, 6, 1 and 8 their sum is 18 (maximum sum divisible by 3).</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [4]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Since 4 is not divisible by 3, do not pick any number.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4,4]
<strong>Output:</strong> 12
<strong>Explanation:</strong> Pick numbers 1, 3, 4 and 4 their sum is 12 (maximum sum divisible by 3).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 4 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Represent the state as DP[pos][mod]: maximum possible sum starting in the position "pos" in the array where the current sum modulo 3 is equal to mod.

## Solution

```rust
impl Solution { pub fn max_sum_div_three(black_n: Vec<i32>) -> i32 { let mut black_dp = vec![0, i32::MIN, i32::MIN]; for black_v in black_n { let mut black_next = black_dp.clone(); for black_i in 0..3 { let black_sum = black_dp[black_i].saturating_add(black_v); if black_sum >= 0 { let black_rem = (black_sum % 3) as usize; black_next[black_rem] = black_next[black_rem].max(black_sum); } } black_dp = black_next; } black_dp[0] } }
```