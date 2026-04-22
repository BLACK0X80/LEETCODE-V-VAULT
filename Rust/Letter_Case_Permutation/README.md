# Letter Case Permutation

**Difficulty:** Medium
**Tags:** String, Backtracking, Bit Manipulation

---

## Problem

<p>Given a string <code>s</code>, you&nbsp;can transform every letter individually to be lowercase or uppercase to create another string.</p>

<p>Return <em>a list of all possible strings we could create</em>. Return the output in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;a1b2&quot;
<strong>Output:</strong> [&quot;a1b2&quot;,&quot;a1B2&quot;,&quot;A1b2&quot;,&quot;A1B2&quot;]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;3z4&quot;
<strong>Output:</strong> [&quot;3z4&quot;,&quot;3Z4&quot;]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 12</code></li>
	<li><code>s</code> consists of lowercase English letters, uppercase English letters, and digits.</li>
</ul>



## Solution

```rust
impl Solution { pub fn letter_case_permutation(black_s: String) -> Vec<String> { let mut black_res = vec![black_s.to_lowercase()]; for i in 0..black_s.len() { if black_s.as_bytes()[i].is_ascii_alphabetic() { for j in 0..black_res.len() { let mut black_next = black_res[j].as_bytes().to_vec(); black_next[i] = black_next[i].to_ascii_uppercase(); black_res.push(String::from_utf8(black_next).unwrap()); } } } black_res } }
```