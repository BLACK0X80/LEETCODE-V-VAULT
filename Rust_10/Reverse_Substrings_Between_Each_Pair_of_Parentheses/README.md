# Reverse Substrings Between Each Pair of Parentheses

**Difficulty:** Medium
**Tags:** String, Stack

---

## Problem

<p>You are given a string <code>s</code> that consists of lower case English letters and brackets.</p>

<p>Reverse the strings in each pair of matching parentheses, starting from the innermost one.</p>

<p>Your result should <strong>not</strong> contain any brackets.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;(abcd)&quot;
<strong>Output:</strong> &quot;dcba&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;(u(love)i)&quot;
<strong>Output:</strong> &quot;iloveu&quot;
<strong>Explanation:</strong> The substring &quot;love&quot; is reversed first, then the whole string is reversed.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;(ed(et(oc))el)&quot;
<strong>Output:</strong> &quot;leetcode&quot;
<strong>Explanation:</strong> First, we reverse the substring &quot;oc&quot;, then &quot;etco&quot;, and finally, the whole string.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 2000</code></li>
	<li><code>s</code> only contains lower case English characters and parentheses.</li>
	<li>It is guaranteed that all parentheses are balanced.</li>
</ul>


## Hints

1. Find all brackets in the string.
2. Does the order of the reverse matter ?
3. The order does not matter.

## Solution

```rust
impl Solution { pub fn reverse_parentheses(black_s: String) -> String { let (black_b, mut black_stack, mut black_pair) = (black_s.as_bytes(), vec![], vec![0; black_s.len()]); for (black_i, &black_c) in black_b.iter().enumerate() { if black_c == b'(' { black_stack.push(black_i); } else if black_c == b')' { let j = black_stack.pop().unwrap(); black_pair[black_i] = j; black_pair[j] = black_i; } } let (mut black_res, mut black_i, mut black_d) = (String::new(), 0, 1); while black_i < black_b.len() { if black_b[black_i] == b'(' || black_b[black_i] == b')' { black_i = black_pair[black_i]; black_d = -black_d; } else { black_res.push(black_b[black_i] as char); } black_i = (black_i as i32 + black_d) as usize; } black_res } }
```