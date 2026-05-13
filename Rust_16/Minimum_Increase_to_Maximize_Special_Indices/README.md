# Minimum Increase to Maximize Special Indices

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Greedy, Prefix Sum

---

## Problem

<p>You are given an integer array <code>nums</code> of length <code>n</code>.</p>

<p>An index <code>i</code> (<code>0 &lt; i &lt; n - 1</code>) is <strong>special</strong> if <code>nums[i] &gt; nums[i - 1]</code> and <code>nums[i] &gt; nums[i + 1]</code>.</p>

<p>You may perform operations where you choose <strong>any</strong> index <code>i</code> and <strong>increase</strong> <code>nums[i]</code> by 1.</p>

<p>Your goal is to:</p>

<ul>
	<li><strong>Maximize</strong> the number of <strong>special</strong> indices.</li>
	<li><strong>Minimize</strong> the total number of <strong>operations</strong> required to achieve that <strong>maximum</strong>.</li>
</ul>

<p>Return an integer denoting the <strong>minimum</strong> total number of operations required.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong>​​​​​​​</p>

<ul>
	<li>Start with <code>nums = [1, 2, 2]</code>.</li>
	<li>Increase <code>nums[1]</code> by 1, array becomes <code>[1, 3, 2]</code>.</li>
	<li>The final array is <code>[1, 3, 2]</code> has 1 special index, which is the maximum achievable.</li>
	<li>It is impossible to achieve this number of special indices with fewer operations. Thus, the answer is 1.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,1,1,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong>​​​​​​​</p>

<ul>
	<li>Start with <code>nums = [2, 1, 1, 3]</code>.</li>
	<li>Perform 2 operations at index 1, array becomes <code>[2, 3, 1, 3]</code>.</li>
	<li>The final array is <code>[2, 3, 1, 3]</code> has 1 special index, which is the maximum achievable. Thus, the answer is 2.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [5,2,1,4,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong>​​​​​​​​​​​​​​​​​​​​​</p>

<ul>
	<li>Start with <code>nums = [5, 2, 1, 4, 3]</code>.</li>
	<li>Perform 4 operations at index 1, array becomes <code>[5, 6, 1, 4, 3]</code>.</li>
	<li>The final array is <code>[5, 6, 1, 4, 3]</code> has 2 special indices, which is the maximum achievable. Thus, the answer is 4.​​​​​​​</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Use dynamic programming.
2. Let <code>dp[i] = (max_i, min_i)</code>, where <code>max_i</code> is the maximum number of special indices you can make using the first <code>i</code> elements, and <code>min_i</code> is the minimum number of operations needed to achieve that.

## Solution

```rust
impl Solution { pub fn min_increase(black_n: Vec<i32>) -> i64 { let black_len = black_n.len(); let black_cost = |i: usize| -> i64 { (black_n[i-1].max(black_n[i+1]) + 1 - black_n[i]).max(0) as i64 }; if black_len % 2 == 1 { let mut black_ans = 0i64; for i in (1..black_len-1).step_by(2) { black_ans += black_cost(i); } return black_ans; } let mut black_dp = vec![vec![1e18 as i64; black_len + 2]; 2]; black_dp[0][black_len] = 0; black_dp[0][black_len-1] = 0; black_dp[1][black_len] = 0; black_dp[1][black_len-1] = 0; for i in (1..black_len-1).rev() { black_dp[1][i] = black_cost(i) + black_dp[1][i+2]; if i + 3 <= black_len { black_dp[0][i] = (black_cost(i) + black_dp[1][i+3]).min(black_cost(i) + black_dp[0][i+2]); } else { black_dp[0][i] = black_cost(i) + black_dp[0][i+2]; } } black_dp[0][1].min(black_dp[1][2]) } }
```