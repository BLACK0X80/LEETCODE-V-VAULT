# Longest Palindromic Subsequence

**Difficulty:** Medium
**Tags:** String, Dynamic Programming

---

## Problem

<p>Given a string <code>s</code>, find <em>the longest palindromic <strong>subsequence</strong>&#39;s length in</em> <code>s</code>.</p>

<p>A <strong>subsequence</strong> is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;bbbab&quot;
<strong>Output:</strong> 4
<strong>Explanation:</strong> One possible longest palindromic subsequence is &quot;bbbb&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;cbbd&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> One possible longest palindromic subsequence is &quot;bb&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 1000</code></li>
	<li><code>s</code> consists only of lowercase English letters.</li>
</ul>



## Solution

```rust
impl Solution { pub fn longest_palindrome_subseq(black_s: String) -> i32 { let black_b = black_s.as_bytes(); let black_n = black_b.len(); let mut black_dp = vec![0; black_n]; for black_i in (0..black_n).rev() { black_dp[black_i] = 1; let mut black_prev = 0; for black_j in black_i + 1..black_n { let black_tmp = black_dp[black_j]; if black_b[black_i] == black_b[black_j] { black_dp[black_j] = black_prev + 2; } else { black_dp[black_j] = black_dp[black_j].max(black_dp[black_j - 1]); } black_prev = black_tmp; } } black_dp[black_n - 1] } }
```