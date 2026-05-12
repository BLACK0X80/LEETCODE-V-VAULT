# Shortest String That Contains Three Strings

**Difficulty:** Medium
**Tags:** String, Greedy, Enumeration

---

## Problem

Given three strings <code>a</code>, <code>b</code>, and <code>c</code>, your task is to find a string that has the<strong> minimum</strong> length and contains all three strings as <strong>substrings</strong>.
<p>If there are multiple such strings, return the<em> </em><strong>lexicographically<em> </em>smallest </strong>one.</p>

<p>Return <em>a string denoting the answer to the problem.</em></p>

<p><strong>Notes</strong></p>

<ul>
	<li>A string <code>a</code> is <strong>lexicographically smaller</strong> than a string <code>b</code> (of the same length) if in the first position where <code>a</code> and <code>b</code> differ, string <code>a</code> has a letter that appears <strong>earlier </strong>in the alphabet than the corresponding letter in <code>b</code>.</li>
	<li>A <strong>substring</strong> is a contiguous sequence of characters within a string.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> a = &quot;abc&quot;, b = &quot;bca&quot;, c = &quot;aaa&quot;
<strong>Output:</strong> &quot;aaabca&quot;
<strong>Explanation:</strong>  We show that &quot;aaabca&quot; contains all the given strings: a = ans[2...4], b = ans[3..5], c = ans[0..2]. It can be shown that the length of the resulting string would be at least 6 and &quot;aaabca&quot; is the lexicographically smallest one.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> a = &quot;ab&quot;, b = &quot;ba&quot;, c = &quot;aba&quot;
<strong>Output:</strong> &quot;aba&quot;
<strong>Explanation: </strong>We show that the string &quot;aba&quot; contains all the given strings: a = ans[0..1], b = ans[1..2], c = ans[0..2]. Since the length of c is 3, the length of the resulting string would be at least 3. It can be shown that &quot;aba&quot; is the lexicographically smallest one.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= a.length, b.length, c.length &lt;= 100</code></li>
	<li><code>a</code>, <code>b</code>, <code>c</code> consist only of lowercase English letters.</li>
</ul>


## Hints

1. Think about how you can generate all possible strings that contain all three input strings as substrings. Can you come up with an efficient algorithm to do this?
2. Check all permutations of the words a, b, and c. For each permutation, begin by appending some letters to the end of the first word to form the second word. Then, proceed to add more letters to generate the third word.

## Solution

```rust
impl Solution { pub fn minimum_string(a: String, b: String, c: String) -> String { let black_merge = |s1: &str, s2: &str| { if s1.contains(s2) { return s1.to_string(); } for i in (0..s1.len().min(s2.len())).rev() { if s1.ends_with(&s2[..=i]) { return format!("{}{}", s1, &s2[i+1..]); } } format!("{}{}", s1, s2) }; let mut black_res = String::new(); let black_v = vec![&a, &b, &c]; let mut black_perms = vec![0, 1, 2]; loop { let black_s = black_merge(&black_merge(black_v[black_perms[0]], black_v[black_perms[1]]), black_v[black_perms[2]]); if black_res.is_empty() || black_s.len() < black_res.len() || (black_s.len() == black_res.len() && black_s < black_res) { black_res = black_s; } if !Self::next_perm(&mut black_perms) { break; } } black_res } fn next_perm(p: &mut Vec<usize>) -> bool { let i = match (0..p.len()-1).rev().find(|&i| p[i] < p[i+1]) { Some(idx) => idx, None => return false }; let j = (i+1..p.len()).rev().find(|&j| p[i] < p[j]).unwrap(); p.swap(i, j); p[i+1..].reverse(); true } }
```