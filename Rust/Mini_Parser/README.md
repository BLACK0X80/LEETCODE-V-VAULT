# Mini Parser

**Difficulty:** Medium
**Tags:** String, Stack, Depth-First Search

---

## Problem

<p>Given a string s represents the serialization of a nested list, implement a parser to deserialize it and return <em>the deserialized</em> <code>NestedInteger</code>.</p>

<p>Each element is either an integer or a list whose elements may also be integers or other lists.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;324&quot;
<strong>Output:</strong> 324
<strong>Explanation:</strong> You should return a NestedInteger object which contains a single integer 324.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;[123,[456,[789]]]&quot;
<strong>Output:</strong> [123,[456,[789]]]
<strong>Explanation:</strong> Return a NestedInteger object containing a nested list with 2 elements:
1. An integer containing value 123.
2. A nested list containing two elements:
    i.  An integer containing value 456.
    ii. A nested list with one element:
         a. An integer containing value 789
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>s</code> consists of digits, square brackets <code>&quot;[]&quot;</code>, negative sign <code>&#39;-&#39;</code>, and commas <code>&#39;,&#39;</code>.</li>
	<li><code>s</code> is the serialization of valid <code>NestedInteger</code>.</li>
	<li>All the values in the input are in the range <code>[-10<sup>6</sup>, 10<sup>6</sup>]</code>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn deserialize(black_s: String) -> NestedInteger { if !black_s.starts_with('[') { return NestedInteger::Int(black_s.parse().unwrap()); } let (mut black_stack, mut black_num, mut black_has_num) = (vec![], String::new(), false); for c in black_s.chars() { match c { '[' => black_stack.push(NestedInteger::List(vec![])), '-' | '0'..='9' => { black_num.push(c); black_has_num = true; } ',' | ']' => { if black_has_num { let val = NestedInteger::Int(black_num.parse().unwrap()); if let Some(NestedInteger::List(l)) = black_stack.last_mut() { l.push(val); } black_num.clear(); black_has_num = false; } if c == ']' && black_stack.len() > 1 { let last = black_stack.pop().unwrap(); if let Some(NestedInteger::List(l)) = black_stack.last_mut() { l.push(last); } } } _ => {} } } black_stack.pop().unwrap() } }
```