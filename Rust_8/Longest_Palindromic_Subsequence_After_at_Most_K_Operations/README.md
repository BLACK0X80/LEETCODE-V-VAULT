# Longest Palindromic Subsequence After at Most K Operations

**Difficulty:** Medium
**Tags:** String, Dynamic Programming

---

## Problem

<p>You are given a string <code>s</code> and an integer <code>k</code>.</p>

<p>In one operation, you can replace the character at any position with the next or previous letter in the alphabet (wrapping around so that <code>&#39;a&#39;</code> is after <code>&#39;z&#39;</code>). For example, replacing <code>&#39;a&#39;</code> with the next letter results in <code>&#39;b&#39;</code>, and replacing <code>&#39;a&#39;</code> with the previous letter results in <code>&#39;z&#39;</code>. Similarly, replacing <code>&#39;z&#39;</code> with the next letter results in <code>&#39;a&#39;</code>, and replacing <code>&#39;z&#39;</code> with the previous letter results in <code>&#39;y&#39;</code>.</p>

<p>Return the length of the <strong>longest <span data-keyword="palindrome-string">palindromic</span> <span data-keyword="subsequence-string-nonempty">subsequence</span></strong> of <code>s</code> that can be obtained after performing <strong>at most</strong> <code>k</code> operations.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abced&quot;, k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Replace <code>s[1]</code> with the next letter, and <code>s</code> becomes <code>&quot;acced&quot;</code>.</li>
	<li>Replace <code>s[4]</code> with the previous letter, and <code>s</code> becomes <code>&quot;accec&quot;</code>.</li>
</ul>

<p>The subsequence <code>&quot;ccc&quot;</code> forms a palindrome of length 3, which is the maximum.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;</span>aaazzz<span class="example-io">&quot;, k = 4</span></p>

<p><strong>Output:</strong> 6</p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Replace <code>s[0]</code> with the previous letter, and <code>s</code> becomes <code>&quot;zaazzz&quot;</code>.</li>
	<li>Replace <code>s[4]</code> with the next letter, and <code>s</code> becomes <code>&quot;zaazaz&quot;</code>.</li>
	<li>Replace <code>s[3]</code> with the next letter, and <code>s</code> becomes <code>&quot;zaaaaz&quot;</code>.</li>
</ul>

<p>The entire string forms a palindrome of length 6.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 200</code></li>
	<li><code>1 &lt;= k &lt;= 200</code></li>
	<li><code>s</code> consists of only lowercase English letters.</li>
</ul>


## Hints

1. Use dynamic programming.
2. <code>dp[i][j][k]</code> is the length of the longest palindromic subsequence in substring <code>[i..j]</code> with cost at most <code>k</code>.
3. <code>dp[i][j][k] = max(dp[i + 1][j][k], dp[i][j - 1][k], dp[i + 1][j - 1][k - dist(s[i], s[j])] + 2)</code>, where <code>dist(x, y)</code> is the minimum cyclic distance between <code>x</code> and <code>y</code>.

## Solution

```rust
impl Solution { pub fn longest_palindromic_subsequence(black_s: String, black_k: i32) -> i32 { let black_b = black_s.as_bytes(); let black_n = black_b.len(); let mut black_dp = vec![vec![vec![0; black_k as usize + 1]; black_n]; black_n]; for black_i in (0..black_n).rev() { for black_j in black_i..black_n { for black_z in 0..=black_k as usize { if black_i == black_j { black_dp[black_i][black_j][black_z] = 1; continue; } let black_d = (black_b[black_i] as i32 - black_b[black_j] as i32).abs(); let black_cost = black_d.min(26 - black_d) as usize; black_dp[black_i][black_j][black_z] = black_dp[black_i + 1][black_j][black_z].max(black_dp[black_i][black_j - 1][black_z]); if black_z >= black_cost { black_dp[black_i][black_j][black_z] = black_dp[black_i][black_j][black_z].max(black_dp[black_i + 1][black_j - 1][black_z - black_cost] + 2); } } } } black_dp[0][black_n - 1][black_k as usize] } }
```