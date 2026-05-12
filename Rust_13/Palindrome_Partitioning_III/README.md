# Palindrome Partitioning III

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>You are given a string <code>s</code> containing lowercase letters and an integer <code>k</code>. You need to :</p>

<ul>
	<li>First, change some characters of <code>s</code> to other lowercase English letters.</li>
	<li>Then divide <code>s</code> into <code>k</code> non-empty disjoint substrings such that each substring is a palindrome.</li>
</ul>

<p>Return <em>the minimal number of characters that you need to change to divide the string</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abc&quot;, k = 2
<strong>Output:</strong> 1
<strong>Explanation:</strong>&nbsp;You can split the string into &quot;ab&quot; and &quot;c&quot;, and change 1 character in &quot;ab&quot; to make it palindrome.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aabbc&quot;, k = 3
<strong>Output:</strong> 0
<strong>Explanation:</strong>&nbsp;You can split the string into &quot;aa&quot;, &quot;bb&quot; and &quot;c&quot;, all of them are palindrome.</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;leetcode&quot;, k = 8
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= s.length &lt;= 100</code>.</li>
	<li><code>s</code> only contains lowercase English letters.</li>
</ul>


## Hints

1. For each substring calculate the minimum number of steps to make it palindrome and store it in a table.
2. Create a dp(pos, cnt) which means the minimum number of characters changed for the suffix of s starting on pos splitting the suffix on cnt chunks.

## Solution

```rust
impl Solution {
    pub fn palindrome_partition(black_s: String, black_k: i32) -> i32 {
        let black_n = black_s.len();
        let black_bytes = black_s.as_bytes();
        let black_k = black_k as usize;
        let mut black_cost = vec![vec![0; black_n]; black_n];

        for black_len in 2..=black_n {
            for black_i in 0..=black_n - black_len {
                let black_j = black_i + black_len - 1;
                black_cost[black_i][black_j] = black_cost[black_i + 1][black_j - 1] + if black_bytes[black_i] == black_bytes[black_j] { 0 } else { 1 };
            }
        }

        let mut black_dp = vec![vec![100; black_k + 1]; black_n + 1];
        black_dp[0][0] = 0;

        for black_i in 1..=black_n {
            for black_j in 1..=black_k.min(black_i) {
                for black_p in 0..black_i {
                    black_dp[black_i][black_j] = black_dp[black_i][black_j].min(black_dp[black_p][black_j - 1] + black_cost[black_p][black_i - 1]);
                }
            }
        }

        black_dp[black_n][black_k] as i32
    }
}
```