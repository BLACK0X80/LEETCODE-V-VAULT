# Find Beautiful Indices in the Given Array I

**Difficulty:** Medium
**Tags:** Two Pointers, String, Binary Search, Rolling Hash, String Matching, Hash Function

---

## Problem

<p>You are given a <strong>0-indexed</strong> string <code>s</code>, a string <code>a</code>, a string <code>b</code>, and an integer <code>k</code>.</p>

<p>An index <code>i</code> is <strong>beautiful</strong> if:</p>

<ul>
	<li><code>0 &lt;= i &lt;= s.length - a.length</code></li>
	<li><code>s[i..(i + a.length - 1)] == a</code></li>
	<li>There exists an index <code>j</code> such that:
	<ul>
		<li><code>0 &lt;= j &lt;= s.length - b.length</code></li>
		<li><code>s[j..(j + b.length - 1)] == b</code></li>
		<li><code>|j - i| &lt;= k</code></li>
	</ul>
	</li>
</ul>

<p>Return <em>the array that contains beautiful indices in <strong>sorted order from smallest to largest</strong></em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;isawsquirrelnearmysquirrelhouseohmy&quot;, a = &quot;my&quot;, b = &quot;squirrel&quot;, k = 15
<strong>Output:</strong> [16,33]
<strong>Explanation:</strong> There are 2 beautiful indices: [16,33].
- The index 16 is beautiful as s[16..17] == &quot;my&quot; and there exists an index 4 with s[4..11] == &quot;squirrel&quot; and |16 - 4| &lt;= 15.
- The index 33 is beautiful as s[33..34] == &quot;my&quot; and there exists an index 18 with s[18..25] == &quot;squirrel&quot; and |33 - 18| &lt;= 15.
Thus we return [16,33] as the result.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcd&quot;, a = &quot;a&quot;, b = &quot;a&quot;, k = 4
<strong>Output:</strong> [0]
<strong>Explanation:</strong> There is 1 beautiful index: [0].
- The index 0 is beautiful as s[0..0] == &quot;a&quot; and there exists an index 0 with s[0..0] == &quot;a&quot; and |0 - 0| &lt;= 4.
Thus we return [0] as the result.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= a.length, b.length &lt;= 10</code></li>
	<li><code>s</code>, <code>a</code>, and <code>b</code> contain only lowercase English letters.</li>
</ul>


## Hints

1. For each <code>i</code>, you can iterate over all <code>j</code>s and determine if <code>i</code> is beautiful or not.

## Solution

```rust
impl Solution { pub fn beautiful_indices(black_s: String, black_a: String, black_b: String, black_k: i32) -> Vec<i32> { let (mut black_pos_a, mut black_pos_b) = (vec![], vec![]); if black_s.len() < black_a.len() || black_s.len() < black_b.len() { return vec![]; } for i in 0..=black_s.len() - black_a.len() { if &black_s[i..i + black_a.len()] == black_a { black_pos_a.push(i as i32); } } for j in 0..=black_s.len() - black_b.len() { if &black_s[j..j + black_b.len()] == black_b { black_pos_b.push(j as i32); } } black_pos_a.into_iter().filter(|&i| { let black_idx = black_pos_b.binary_search(&i).unwrap_or_else(|x| x); (black_idx < black_pos_b.len() && black_pos_b[black_idx] - i <= black_k) || (black_idx > 0 && i - black_pos_b[black_idx - 1] <= black_k) }).collect() } }
```