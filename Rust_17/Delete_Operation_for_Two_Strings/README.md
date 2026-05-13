# Delete Operation for Two Strings

**Difficulty:** Medium
**Tags:** String, Dynamic Programming

---

## Problem

<p>Given two strings <code>word1</code> and <code>word2</code>, return <em>the minimum number of <strong>steps</strong> required to make</em> <code>word1</code> <em>and</em> <code>word2</code> <em>the same</em>.</p>

<p>In one <strong>step</strong>, you can delete exactly one character in either string.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> word1 = &quot;sea&quot;, word2 = &quot;eat&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> You need one step to make &quot;sea&quot; to &quot;ea&quot; and another step to make &quot;eat&quot; to &quot;ea&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> word1 = &quot;leetcode&quot;, word2 = &quot;etco&quot;
<strong>Output:</strong> 4
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= word1.length, word2.length &lt;= 500</code></li>
	<li><code>word1</code> and <code>word2</code> consist of only lowercase English letters.</li>
</ul>



## Solution

```rust
impl Solution { pub fn min_distance(w1: String, w2: String) -> i32 { let (b1, b2) = (w1.as_bytes(), w2.as_bytes()); let mut black_dp = vec![0; b2.len() + 1]; for i in 0..b1.len() { let mut black_prev = 0; for j in 1..=b2.len() { let black_tmp = black_dp[j]; if b1[i] == b2[j-1] { black_dp[j] = black_prev + 1; } else { black_dp[j] = black_dp[j].max(black_dp[j-1]); } black_prev = black_tmp; } } (b1.len() + b2.len() - 2 * black_dp[b2.len()] as usize) as i32 } }
```