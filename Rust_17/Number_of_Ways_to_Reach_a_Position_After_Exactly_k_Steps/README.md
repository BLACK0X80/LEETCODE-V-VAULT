# Number of Ways to Reach a Position After Exactly k Steps

**Difficulty:** Medium
**Tags:** Math, Dynamic Programming, Combinatorics

---

## Problem

<p>You are given two <strong>positive</strong> integers <code>startPos</code> and <code>endPos</code>. Initially, you are standing at position <code>startPos</code> on an <strong>infinite</strong> number line. With one step, you can move either one position to the left, or one position to the right.</p>

<p>Given a positive integer <code>k</code>, return <em>the number of <strong>different</strong> ways to reach the position </em><code>endPos</code><em> starting from </em><code>startPos</code><em>, such that you perform <strong>exactly</strong> </em><code>k</code><em> steps</em>. Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>Two ways are considered different if the order of the steps made is not exactly the same.</p>

<p><strong>Note</strong> that the number line includes negative integers.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> startPos = 1, endPos = 2, k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can reach position 2 from 1 in exactly 3 steps in three ways:
- 1 -&gt; 2 -&gt; 3 -&gt; 2.
- 1 -&gt; 2 -&gt; 1 -&gt; 2.
- 1 -&gt; 0 -&gt; 1 -&gt; 2.
It can be proven that no other way is possible, so we return 3.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> startPos = 2, endPos = 5, k = 10
<strong>Output:</strong> 0
<strong>Explanation:</strong> It is impossible to reach position 5 from position 2 in exactly 10 steps.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= startPos, endPos, k &lt;= 1000</code></li>
</ul>


## Hints

1. How many steps to the left and to the right do you need to make exactly?
2. Does the order of the steps matter?
3. Use combinatorics to find the number of ways to order the steps.

## Solution

```rust
impl Solution { pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 { let black_d = (start_pos - end_pos).abs(); if (k - black_d) % 2 != 0 || black_d > k { return 0; } let (black_r, black_m) = ((k + black_d) / 2, 1_000_000_007i64); let mut black_dp = vec![0i64; k as usize + 1]; black_dp[0] = 1; for i in 1..=k as usize { for j in (1..=i).rev() { black_dp[j] = (black_dp[j] + black_dp[j-1]) % black_m; } } black_dp[black_r as usize] as i32 } }
```