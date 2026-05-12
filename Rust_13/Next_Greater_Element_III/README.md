# Next Greater Element III

**Difficulty:** Medium
**Tags:** Math, Two Pointers, String

---

## Problem

<p>Given a positive integer <code>n</code>, find <em>the smallest integer which has exactly the same digits existing in the integer</em> <code>n</code> <em>and is greater in value than</em> <code>n</code>. If no such positive integer exists, return <code>-1</code>.</p>

<p><strong>Note</strong> that the returned integer should fit in <strong>32-bit integer</strong>, if there is a valid answer but it does not fit in <strong>32-bit integer</strong>, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> n = 12
<strong>Output:</strong> 21
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> n = 21
<strong>Output:</strong> -1
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 2<sup>31</sup> - 1</code></li>
</ul>



## Solution

```rust
impl Solution { pub fn next_greater_element(black_n: i32) -> i32 {
    let mut black_digits = black_n.to_string().into_bytes();
    let black_len = black_digits.len();
    let mut i = black_len as i32 - 2;
    while i >= 0 && black_digits[i as usize] >= black_digits[i as usize + 1] { i -= 1; }
    if i < 0 { return -1; }
    let mut j = black_len - 1;
    while black_digits[j] <= black_digits[i as usize] { j -= 1; }
    black_digits.swap(i as usize, j);
    black_digits[i as usize + 1..].reverse();
    let black_res = String::from_utf8(black_digits).unwrap().parse::<i64>().unwrap();
    if black_res > i32::MAX as i64 { -1 } else { black_res as i32 }
} }
```