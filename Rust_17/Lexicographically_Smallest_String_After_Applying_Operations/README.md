# Lexicographically Smallest String After Applying Operations

**Difficulty:** Medium
**Tags:** String, Depth-First Search, Breadth-First Search, Enumeration

---

## Problem

<p>You are given a string <code>s</code> of <strong>even length</strong> consisting of digits from <code>0</code> to <code>9</code>, and two integers <code>a</code> and <code>b</code>.</p>

<p>You can apply either of the following two operations any number of times and in any order on <code>s</code>:</p>

<ul>
	<li>Add <code>a</code> to all odd indices of <code>s</code> <strong>(0-indexed)</strong>. Digits post <code>9</code> are cycled back to <code>0</code>. For example, if <code>s = &quot;3456&quot;</code> and <code>a = 5</code>, <code>s</code> becomes <code>&quot;3951&quot;</code>.</li>
	<li>Rotate <code>s</code> to the right by <code>b</code> positions. For example, if <code>s = &quot;3456&quot;</code> and <code>b = 1</code>, <code>s</code> becomes <code>&quot;6345&quot;</code>.</li>
</ul>

<p>Return <em>the <strong>lexicographically smallest</strong> string you can obtain by applying the above operations any number of times on</em> <code>s</code>.</p>

<p>A string <code>a</code> is lexicographically smaller than a string <code>b</code> (of the same length) if in the first position where <code>a</code> and <code>b</code> differ, string <code>a</code> has a letter that appears earlier in the alphabet than the corresponding letter in <code>b</code>. For example, <code>&quot;0158&quot;</code> is lexicographically smaller than <code>&quot;0190&quot;</code> because the first position they differ is at the third letter, and <code>&#39;5&#39;</code> comes before <code>&#39;9&#39;</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;5525&quot;, a = 9, b = 2
<strong>Output:</strong> &quot;2050&quot;
<strong>Explanation:</strong> We can apply the following operations:
Start:  &quot;5525&quot;
Rotate: &quot;2555&quot;
Add:    &quot;2454&quot;
Add:    &quot;2353&quot;
Rotate: &quot;5323&quot;
Add:    &quot;5222&quot;
Add:    &quot;5121&quot;
Rotate: &quot;2151&quot;
Add:    &quot;2050&quot;​​​​​
There is no way to obtain a string that is lexicographically smaller than &quot;2050&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;74&quot;, a = 5, b = 1
<strong>Output:</strong> &quot;24&quot;
<strong>Explanation:</strong> We can apply the following operations:
Start:  &quot;74&quot;
Rotate: &quot;47&quot;
​​​​​​​Add:    &quot;42&quot;
​​​​​​​Rotate: &quot;24&quot;​​​​​​​​​​​​
There is no way to obtain a string that is lexicographically smaller than &quot;24&quot;.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;0011&quot;, a = 4, b = 2
<strong>Output:</strong> &quot;0011&quot;
<strong>Explanation:</strong> There are no sequence of operations that will give us a lexicographically smaller string than &quot;0011&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= s.length &lt;= 100</code></li>
	<li><code>s.length</code> is even.</li>
	<li><code>s</code> consists of digits from <code>0</code> to <code>9</code> only.</li>
	<li><code>1 &lt;= a &lt;= 9</code></li>
	<li><code>1 &lt;= b &lt;= s.length - 1</code></li>
</ul>


## Hints

1. Since the length of s is even, the total number of possible sequences is at most 10 * 10 * s.length.
2. You can generate all possible sequences and take their minimum.
3. Keep track of already generated sequences so they are not processed again.

## Solution

```rust
impl Solution { pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String { let (mut black_q, mut black_vis, mut black_res) = (std::collections::VecDeque::from([s.clone()]), std::collections::HashSet::from([s.clone()]), s.clone()); while let Some(black_curr) = black_q.pop_front() { if black_curr < black_res { black_res = black_curr.clone(); } let mut black_add = black_curr.as_bytes().to_vec(); for black_i in (1..black_add.len()).step_by(2) { black_add[black_i] = b'0' + (black_add[black_i] - b'0' + a as u8) % 10; } let black_s_add = String::from_utf8(black_add).unwrap(); if black_vis.insert(black_s_add.clone()) { black_q.push_back(black_s_add); } let black_s_rot = format!("{}{}", &black_curr[black_curr.len() - b as usize..], &black_curr[..black_curr.len() - b as usize]); if black_vis.insert(black_s_rot.clone()) { black_q.push_back(black_s_rot); } } black_res } }
```