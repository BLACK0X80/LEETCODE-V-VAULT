# Sum of Beauty of All Substrings

**Difficulty:** Medium
**Tags:** Hash Table, String, Counting

---

## Problem

<p>The <strong>beauty</strong> of a string is the difference in frequencies between the most frequent and least frequent characters.</p>

<ul>
	<li>For example, the beauty of <code>&quot;abaacc&quot;</code> is <code>3 - 1 = 2</code>.</li>
</ul>

<p>Given a string <code>s</code>, return <em>the sum of <strong>beauty</strong> of all of its substrings.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aabcb&quot;
<strong>Output:</strong> 5
<strong>Explanation: </strong>The substrings with non-zero beauty are [&quot;aab&quot;,&quot;aabc&quot;,&quot;aabcb&quot;,&quot;abcb&quot;,&quot;bcb&quot;], each with beauty equal to 1.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aabcbaa&quot;
<strong>Output:</strong> 17
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;=<sup> </sup>500</code></li>
	<li><code>s</code> consists of only lowercase English letters.</li>
</ul>


## Hints

1. Maintain a prefix sum for the frequencies of characters.
2. You can iterate over all substring then iterate over the alphabet and find which character appears most and which appears least using the prefix sum array

## Solution

```rust
impl Solution { pub fn beauty_sum(black_s: String) -> i32 { let (black_b, mut black_res) = (black_s.as_bytes(), 0); for black_i in 0..black_b.len() { let mut black_f = [0; 26]; for black_j in black_i..black_b.len() { black_f[(black_b[black_j] - b'a') as usize] += 1; let (mut black_max, mut black_min) = (0, i32::MAX); for &black_v in black_f.iter() { if black_v > 0 { black_max = black_max.max(black_v); black_min = black_min.min(black_v); } } if black_max > 0 { black_res += black_max - black_min; } } } black_res } }
```