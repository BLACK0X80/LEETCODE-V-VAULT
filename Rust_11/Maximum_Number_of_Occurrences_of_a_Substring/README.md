# Maximum Number of Occurrences of a Substring

**Difficulty:** Medium
**Tags:** Hash Table, String, Sliding Window

---

## Problem

<p>Given a string <code>s</code>, return the maximum number of occurrences of <strong>any</strong> substring under the following rules:</p>

<ul>
	<li>The number of unique characters in the substring must be less than or equal to <code>maxLetters</code>.</li>
	<li>The substring size must be between <code>minSize</code> and <code>maxSize</code> inclusive.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aababcaab&quot;, maxLetters = 2, minSize = 3, maxSize = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong> Substring &quot;aab&quot; has 2 occurrences in the original string.
It satisfies the conditions, 2 unique letters and size 3 (between minSize and maxSize).
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aaaa&quot;, maxLetters = 1, minSize = 3, maxSize = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> Substring &quot;aaa&quot; occur 2 times in the string. It can overlap.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= maxLetters &lt;= 26</code></li>
	<li><code>1 &lt;= minSize &lt;= maxSize &lt;= min(26, s.length)</code></li>
	<li><code>s</code> consists of only lowercase English letters.</li>
</ul>


## Hints

1. Check out the constraints, (maxSize <=26).
2. This means you can explore all substrings in O(n * 26).
3. Find the Maximum Number of Occurrences of a Substring with bruteforce.

## Solution

```rust
impl Solution { pub fn max_freq(black_s: String, black_l: i32, black_min: i32, _: i32) -> i32 { let (black_b, mut black_cnt, mut black_res, black_m) = (black_s.as_bytes(), std::collections::HashMap::new(), 0, black_min as usize); for black_i in 0..=black_b.len().saturating_sub(black_m) { let black_sub = &black_b[black_i..black_i + black_m]; let mut black_u = 0; let mut black_mask = 0u32; for &black_char in black_sub { let black_bit = (black_char - b'a') as u32; if (black_mask >> black_bit) & 1 == 0 { black_u += 1; black_mask |= (1 << black_bit); } } if black_u <= black_l { let black_e = black_cnt.entry(black_sub).or_insert(0); *black_e += 1; black_res = black_res.max(*black_e); } } black_res } }
```