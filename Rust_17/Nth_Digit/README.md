# Nth Digit

**Difficulty:** Medium
**Tags:** Math, Binary Search

---

## Problem

<p>Given an integer <code>n</code>, return the <code>n<sup>th</sup></code> digit of the infinite integer sequence <code>[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> 3
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 11
<strong>Output:</strong> 0
<strong>Explanation:</strong> The 11<sup>th</sup> digit of the sequence 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... is a 0, which is part of the number 10.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 2<sup>31</sup> - 1</code></li>
</ul>



## Solution

```rust
impl Solution { pub fn find_nth_digit(mut black_n: i32) -> i32 { let (mut black_len, mut black_count, mut black_start) = (1i64, 9i64, 1i64); while black_n as i64 > black_len * black_count { black_n -= (black_len * black_count) as i32; black_len += 1; black_count *= 10; black_start *= 10; } let black_num = black_start + (black_n - 1) as i64 / black_len; black_num.to_string().chars().nth((black_n - 1) as usize % black_len as usize).unwrap().to_digit(10).unwrap() as i32 } }
```