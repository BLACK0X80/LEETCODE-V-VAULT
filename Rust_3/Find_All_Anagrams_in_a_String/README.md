# Find All Anagrams in a String

**Difficulty:** Medium
**Tags:** Hash Table, String, Sliding Window

---

## Problem

<p>Given two strings <code>s</code> and <code>p</code>, return an array of all the start indices of <code>p</code>&#39;s <span data-keyword="anagram">anagrams</span> in <code>s</code>. You may return the answer in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;cbaebabacd&quot;, p = &quot;abc&quot;
<strong>Output:</strong> [0,6]
<strong>Explanation:</strong>
The substring with start index = 0 is &quot;cba&quot;, which is an anagram of &quot;abc&quot;.
The substring with start index = 6 is &quot;bac&quot;, which is an anagram of &quot;abc&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abab&quot;, p = &quot;ab&quot;
<strong>Output:</strong> [0,1,2]
<strong>Explanation:</strong>
The substring with start index = 0 is &quot;ab&quot;, which is an anagram of &quot;ab&quot;.
The substring with start index = 1 is &quot;ba&quot;, which is an anagram of &quot;ab&quot;.
The substring with start index = 2 is &quot;ab&quot;, which is an anagram of &quot;ab&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length, p.length &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>s</code> and <code>p</code> consist of lowercase English letters.</li>
</ul>



## Solution

```rust
impl Solution { pub fn find_anagrams(black_s: String, black_p: String) -> Vec<i32> { if black_s.len() < black_p.len() { return vec![]; } let (mut black_f, mut black_w, mut black_r, black_sb, black_pb) = ([0i32; 26], [0i32; 26], vec![], black_s.as_bytes(), black_p.as_bytes()); for black_i in 0..black_pb.len() { black_f[(black_pb[black_i] - b'a') as usize] += 1; black_w[(black_sb[black_i] - b'a') as usize] += 1; } if black_f == black_w { black_r.push(0); } for black_i in black_pb.len()..black_sb.len() { black_w[(black_sb[black_i] - b'a') as usize] += 1; black_w[(black_sb[black_i - black_pb.len()] - b'a') as usize] -= 1; if black_f == black_w { black_r.push((black_i - black_pb.len() + 1) as i32); } } black_r } }
```