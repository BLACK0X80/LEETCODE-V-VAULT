# Splitting a String Into Descending Consecutive Values

**Difficulty:** Medium
**Tags:** String, Backtracking, Enumeration

---

## Problem

<p>You are given a string <code>s</code> that consists of only digits.</p>

<p>Check if we can split <code>s</code> into <strong>two or more non-empty substrings</strong> such that the <strong>numerical values</strong> of the substrings are in <strong>descending order</strong> and the <strong>difference</strong> between numerical values of every two <strong>adjacent</strong> <strong>substrings</strong> is equal to <code>1</code>.</p>

<ul>
	<li>For example, the string <code>s = &quot;0090089&quot;</code> can be split into <code>[&quot;0090&quot;, &quot;089&quot;]</code> with numerical values <code>[90,89]</code>. The values are in descending order and adjacent values differ by <code>1</code>, so this way is valid.</li>
	<li>Another example, the string <code>s = &quot;001&quot;</code> can be split into <code>[&quot;0&quot;, &quot;01&quot;]</code>, <code>[&quot;00&quot;, &quot;1&quot;]</code>, or <code>[&quot;0&quot;, &quot;0&quot;, &quot;1&quot;]</code>. However all the ways are invalid because they have numerical values <code>[0,1]</code>, <code>[0,1]</code>, and <code>[0,0,1]</code> respectively, all of which are not in descending order.</li>
</ul>

<p>Return <code>true</code> <em>if it is possible to split</em> <code>s</code>​​​​​​ <em>as described above</em><em>, or </em><code>false</code><em> otherwise.</em></p>

<p>A <strong>substring</strong> is a contiguous sequence of characters in a string.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;1234&quot;
<strong>Output:</strong> false
<strong>Explanation:</strong> There is no valid way to split s.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;050043&quot;
<strong>Output:</strong> true
<strong>Explanation:</strong> s can be split into [&quot;05&quot;, &quot;004&quot;, &quot;3&quot;] with numerical values [5,4,3].
The values are in descending order with adjacent values differing by 1.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;9080701&quot;
<strong>Output:</strong> false
<strong>Explanation:</strong> There is no valid way to split s.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 20</code></li>
	<li><code>s</code> only consists of digits.</li>
</ul>


## Hints

1. One solution is to try all possible splits using backtrack
2. Look out for trailing zeros in string

## Solution

```rust
impl Solution { pub fn split_string(black_s: String) -> bool { fn black_dfs(idx: usize, prev: u64, s: &[u8]) -> bool { if idx == s.len() { return true; } let mut black_val = 0; for i in idx..s.len() { black_val = black_val * 10 + (s[i] - b'0') as u64; if black_val + 1 == prev && black_dfs(i + 1, black_val, s) { return true; } if black_val >= prev && prev != u64::MAX { break; } } false } let black_b = black_s.as_bytes(); let mut black_v = 0; for i in 0..black_b.len() - 1 { black_v = black_v * 10 + (black_b[i] - b'0') as u64; if black_v > 10000000000000000 { break; } if black_dfs(i + 1, black_v, black_b) { return true; } } false } }
```