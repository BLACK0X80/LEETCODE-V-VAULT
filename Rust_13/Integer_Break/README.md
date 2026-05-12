# Integer Break

**Difficulty:** Medium
**Tags:** Math, Dynamic Programming

---

## Problem

<p>Given an integer <code>n</code>, break it into the sum of <code>k</code> <strong>positive integers</strong>, where <code>k &gt;= 2</code>, and maximize the product of those integers.</p>

<p>Return <em>the maximum product you can get</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 1
<strong>Explanation:</strong> 2 = 1 + 1, 1 &times; 1 = 1.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> 36
<strong>Explanation:</strong> 10 = 3 + 3 + 4, 3 &times; 3 &times; 4 = 36.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 58</code></li>
</ul>


## Hints

1. There is a simple O(n) solution to this problem.
2. You may check the breaking results of <i>n</i> ranging from 7 to 10 to discover the regularities.

## Solution

```rust
impl Solution { pub fn integer_break(black_n: i32) -> i32 { if black_n <= 3 { return black_n - 1; } let (black_q, black_r) = (black_n / 3, black_n % 3); match black_r { 0 => 3i32.pow(black_q as u32), 1 => 3i32.pow(black_q as u32 - 1) * 4, _ => 3i32.pow(black_q as u32) * 2 } } }
```