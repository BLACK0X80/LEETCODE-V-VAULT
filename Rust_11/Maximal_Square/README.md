# Maximal Square

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Matrix

---

## Problem

<p>Given an <code>m x n</code> binary <code>matrix</code> filled with <code>0</code>&#39;s and <code>1</code>&#39;s, <em>find the largest square containing only</em> <code>1</code>&#39;s <em>and return its area</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/26/max1grid.jpg" style="width: 400px; height: 319px;" />
<pre>
<strong>Input:</strong> matrix = [[&quot;1&quot;,&quot;0&quot;,&quot;1&quot;,&quot;0&quot;,&quot;0&quot;],[&quot;1&quot;,&quot;0&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;],[&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;],[&quot;1&quot;,&quot;0&quot;,&quot;0&quot;,&quot;1&quot;,&quot;0&quot;]]
<strong>Output:</strong> 4
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/26/max2grid.jpg" style="width: 165px; height: 165px;" />
<pre>
<strong>Input:</strong> matrix = [[&quot;0&quot;,&quot;1&quot;],[&quot;1&quot;,&quot;0&quot;]]
<strong>Output:</strong> 1
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> matrix = [[&quot;0&quot;]]
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == matrix.length</code></li>
	<li><code>n == matrix[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 300</code></li>
	<li><code>matrix[i][j]</code> is <code>&#39;0&#39;</code> or <code>&#39;1&#39;</code>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn maximal_square(black_m: Vec<Vec<char>>) -> i32 { if black_m.is_empty() { return 0; } let (black_r, black_c) = (black_m.len(), black_m[0].len()); let (mut black_dp, mut black_max, mut black_prev) = (vec![0; black_c + 1], 0, 0); for i in 1..=black_r { for j in 1..=black_c { let black_temp = black_dp[j]; if black_m[i-1][j-1] == '1' { black_dp[j] = black_dp[j].min(black_dp[j-1]).min(black_prev) + 1; black_max = black_max.max(black_dp[j]); } else { black_dp[j] = 0; } black_prev = black_temp; } } black_max * black_max } }
```