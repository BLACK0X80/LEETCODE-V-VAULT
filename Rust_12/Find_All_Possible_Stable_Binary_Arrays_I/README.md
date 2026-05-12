# Find All Possible Stable Binary Arrays I

**Difficulty:** Medium
**Tags:** Dynamic Programming, Prefix Sum

---

## Problem

<p>You are given 3 positive integers <code>zero</code>, <code>one</code>, and <code>limit</code>.</p>

<p>A <span data-keyword="binary-array">binary array</span> <code>arr</code> is called <strong>stable</strong> if:</p>

<ul>
	<li>The number of occurrences of 0 in <code>arr</code> is <strong>exactly </strong><code>zero</code>.</li>
	<li>The number of occurrences of 1 in <code>arr</code> is <strong>exactly</strong> <code>one</code>.</li>
	<li>Each <span data-keyword="subarray-nonempty">subarray</span> of <code>arr</code> with a size greater than <code>limit</code> must contain <strong>both </strong>0 and 1.</li>
</ul>

<p>Return the <em>total</em> number of <strong>stable</strong> binary arrays.</p>

<p>Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">zero = 1, one = 1, limit = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The two possible stable binary arrays are <code>[1,0]</code> and <code>[0,1]</code>, as both arrays have a single 0 and a single 1, and no subarray has a length greater than 2.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">zero = 1, one = 2, limit = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The only possible stable binary array is <code>[1,0,1]</code>.</p>

<p>Note that the binary arrays <code>[1,1,0]</code> and <code>[0,1,1]</code> have subarrays of length 2 with identical elements, hence, they are not stable.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">zero = 3, one = 3, limit = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">14</span></p>

<p><strong>Explanation:</strong></p>

<p>All the possible stable binary arrays are <code>[0,0,1,0,1,1]</code>, <code>[0,0,1,1,0,1]</code>, <code>[0,1,0,0,1,1]</code>, <code>[0,1,0,1,0,1]</code>, <code>[0,1,0,1,1,0]</code>, <code>[0,1,1,0,0,1]</code>, <code>[0,1,1,0,1,0]</code>, <code>[1,0,0,1,0,1]</code>, <code>[1,0,0,1,1,0]</code>, <code>[1,0,1,0,0,1]</code>, <code>[1,0,1,0,1,0]</code>, <code>[1,0,1,1,0,0]</code>, <code>[1,1,0,0,1,0]</code>, and <code>[1,1,0,1,0,0]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= zero, one, limit &lt;= 200</code></li>
</ul>


## Hints

1. Let <code>dp[a][b][c = 0/1][d]</code> be the number of stable arrays with exactly <code>a</code> 0s, <code>b</code> 1s and consecutive <code>d</code> value of <code>c</code>’s at the end.
2. Try each case by appending a 0/1 at last to get the inductions.

## Solution

```rust
impl Solution { pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 { let (z, o, l, m) = (zero as usize, one as usize, limit as usize, 1_000_000_007); let mut black_dp = vec![vec![vec![0i64; 2]; o + 1]; z + 1]; for i in 1..=(z.min(l)) { black_dp[i][0][0] = 1; } for j in 1..=(o.min(l)) { black_dp[0][j][1] = 1; } for i in 1..=z { for j in 1..=o { black_dp[i][j][0] = (black_dp[i-1][j][0] + black_dp[i-1][j][1] - if i > l { black_dp[i-l-1][j][1] } else { 0 } + m as i64) % m as i64; black_dp[i][j][1] = (black_dp[i][j-1][0] + black_dp[i][j-1][1] - if j > l { black_dp[i][j-l-1][0] } else { 0 } + m as i64) % m as i64; } } ((black_dp[z][o][0] + black_dp[z][o][1]) % m as i64) as i32 } }
```