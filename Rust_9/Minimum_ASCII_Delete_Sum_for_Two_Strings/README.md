# Minimum ASCII Delete Sum for Two Strings

**Difficulty:** Medium
**Tags:** String, Dynamic Programming

---

## Problem

<p>Given two strings <code>s1</code> and&nbsp;<code>s2</code>, return <em>the lowest <strong>ASCII</strong> sum of deleted characters to make two strings equal</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;sea&quot;, s2 = &quot;eat&quot;
<strong>Output:</strong> 231
<strong>Explanation:</strong> Deleting &quot;s&quot; from &quot;sea&quot; adds the ASCII value of &quot;s&quot; (115) to the sum.
Deleting &quot;t&quot; from &quot;eat&quot; adds 116 to the sum.
At the end, both strings are equal, and 115 + 116 = 231 is the minimum sum possible to achieve this.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;delete&quot;, s2 = &quot;leet&quot;
<strong>Output:</strong> 403
<strong>Explanation:</strong> Deleting &quot;dee&quot; from &quot;delete&quot; to turn the string into &quot;let&quot;,
adds 100[d] + 101[e] + 101[e] to the sum.
Deleting &quot;e&quot; from &quot;leet&quot; adds 101[e] to the sum.
At the end, both strings are equal to &quot;let&quot;, and the answer is 100+101+101+101 = 403.
If instead we turned both strings into &quot;lee&quot; or &quot;eet&quot;, we would get answers of 433 or 417, which are higher.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s1.length, s2.length &lt;= 1000</code></li>
	<li><code>s1</code> and <code>s2</code> consist of lowercase English letters.</li>
</ul>


## Hints

1. Let dp(i, j) be the answer for inputs s1[i:] and s2[j:].

## Solution

```rust
impl Solution { pub fn minimum_delete_sum(s1: String, s2: String) -> i32 { let (b1, b2) = (s1.as_bytes(), s2.as_bytes()); let mut black_dp = vec![0; b2.len() + 1]; for j in 1..=b2.len() { black_dp[j] = black_dp[j-1] + b2[j-1] as i32; } for i in 1..=b1.len() { let mut black_prev = black_dp[0]; black_dp[0] += b1[i-1] as i32; for j in 1..=b2.len() { let black_tmp = black_dp[j]; if b1[i-1] == b2[j-1] { black_dp[j] = black_prev; } else { black_dp[j] = (black_dp[j] + b1[i-1] as i32).min(black_dp[j-1] + b2[j-1] as i32); } black_prev = black_tmp; } } black_dp[b2.len()] } }
```