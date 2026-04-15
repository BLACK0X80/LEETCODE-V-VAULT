# Beautiful Arrangement

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Backtracking, Bit Manipulation, Bitmask

---

## Problem

<p>Suppose you have <code>n</code> integers labeled <code>1</code> through <code>n</code>. A permutation of those <code>n</code> integers <code>perm</code> (<strong>1-indexed</strong>) is considered a <strong>beautiful arrangement</strong> if for every <code>i</code> (<code>1 &lt;= i &lt;= n</code>), <strong>either</strong> of the following is true:</p>

<ul>
	<li><code>perm[i]</code> is divisible by <code>i</code>.</li>
	<li><code>i</code> is divisible by <code>perm[i]</code>.</li>
</ul>

<p>Given an integer <code>n</code>, return <em>the <strong>number</strong> of the <strong>beautiful arrangements</strong> that you can construct</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 2
<b>Explanation:</b> 
The first beautiful arrangement is [1,2]:
    - perm[1] = 1 is divisible by i = 1
    - perm[2] = 2 is divisible by i = 2
The second beautiful arrangement is [2,1]:
    - perm[1] = 2 is divisible by i = 1
    - i = 2 is divisible by perm[2] = 1
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 15</code></li>
</ul>



## Solution

```rust
impl Solution { pub fn count_arrangement(black_n: i32) -> i32 { let mut black_dp = vec![0; 1 << black_n]; black_dp[0] = 1; for mask in 0..(1 << black_n) { let black_pos = (mask as i32).count_ones() as i32 + 1; for i in 1..=black_n { if mask & (1 << (i - 1)) == 0 && (i % black_pos == 0 || black_pos % i == 0) { black_dp[mask | (1 << (i - 1))] += black_dp[mask]; } } } black_dp[(1 << black_n) - 1] } }
```