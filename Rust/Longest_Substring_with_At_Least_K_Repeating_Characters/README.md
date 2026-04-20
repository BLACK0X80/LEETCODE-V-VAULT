# Longest Substring with At Least K Repeating Characters

**Difficulty:** Medium
**Tags:** Hash Table, String, Divide and Conquer, Sliding Window

---

## Problem

<p>Given a string <code>s</code> and an integer <code>k</code>, return <em>the length of the longest substring of</em> <code>s</code> <em>such that the frequency of each character in this substring is greater than or equal to</em> <code>k</code>.</p>

<p data-pm-slice="1 1 []">if no such substring exists, return 0.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aaabb&quot;, k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> The longest substring is &quot;aaa&quot;, as &#39;a&#39; is repeated 3 times.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;ababbc&quot;, k = 2
<strong>Output:</strong> 5
<strong>Explanation:</strong> The longest substring is &quot;ababb&quot;, as &#39;a&#39; is repeated 2 times and &#39;b&#39; is repeated 3 times.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>4</sup></code></li>
	<li><code>s</code> consists of only lowercase English letters.</li>
	<li><code>1 &lt;= k &lt;= 10<sup>5</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn longest_substring(black_s: String, black_k: i32) -> i32 { let (black_b, mut black_res) = (black_s.as_bytes(), 0); for black_t in 1..=26 { let (mut black_c, mut black_u, mut black_v, mut black_l) = ([0; 26], 0, 0, 0); for black_r in 0..black_b.len() { let black_i = (black_b[black_r] - b'a') as usize; if black_c[black_i] == 0 { black_u += 1; } black_c[black_i] += 1; if black_c[black_i] == black_k { black_v += 1; } while black_u > black_t { let black_j = (black_b[black_l] - b'a') as usize; if black_c[black_j] == black_k { black_v -= 1; } black_c[black_j] -= 1; if black_c[black_j] == 0 { black_u -= 1; } black_l += 1; } if black_u == black_t && black_u == black_v { black_res = black_res.max((black_r - black_l + 1) as i32); } } } black_res } }
```