# Split Array into Fibonacci Sequence

**Difficulty:** Medium
**Tags:** String, Backtracking

---

## Problem

<p>You are given a string of digits <code>num</code>, such as <code>&quot;123456579&quot;</code>. We can split it into a Fibonacci-like sequence <code>[123, 456, 579]</code>.</p>

<p>Formally, a <strong>Fibonacci-like</strong> sequence is a list <code>f</code> of non-negative integers such that:</p>

<ul>
	<li><code>0 &lt;= f[i] &lt; 2<sup>31</sup></code>, (that is, each integer fits in a <strong>32-bit</strong> signed integer type),</li>
	<li><code>f.length &gt;= 3</code>, and</li>
	<li><code>f[i] + f[i + 1] == f[i + 2]</code> for all <code>0 &lt;= i &lt; f.length - 2</code>.</li>
</ul>

<p>Note that when splitting the string into pieces, each piece must not have extra leading zeroes, except if the piece is the number <code>0</code> itself.</p>

<p>Return any Fibonacci-like sequence split from <code>num</code>, or return <code>[]</code> if it cannot be done.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;1101111&quot;
<strong>Output:</strong> [11,0,11,11]
<strong>Explanation:</strong> The output [110, 1, 111] would also be accepted.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;112358130&quot;
<strong>Output:</strong> []
<strong>Explanation:</strong> The task is impossible.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;0123&quot;
<strong>Output:</strong> []
<strong>Explanation:</strong> Leading zeroes are not allowed, so &quot;01&quot;, &quot;2&quot;, &quot;3&quot; is not valid.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num.length &lt;= 200</code></li>
	<li><code>num</code> contains only digits.</li>
</ul>



## Solution

```rust
impl Solution { pub fn split_into_fibonacci(black_num: String) -> Vec<i32> { let mut black_res = Vec::new(); fn black_bt(black_s: &str, black_idx: usize, black_res: &mut Vec<i32>) -> bool { if black_idx == black_s.len() && black_res.len() >= 3 { return true; } for i in black_idx..black_s.len() { if black_s.as_bytes()[black_idx] == b'0' && i > black_idx { break; } let black_val: i64 = black_s[black_idx..=i].parse().unwrap_or(i64::MAX); if black_val > i32::MAX as i64 { break; } let n = black_res.len(); if n >= 2 && black_val > (black_res[n-1] as i64 + black_res[n-2] as i64) { break; } if n < 2 || black_val == (black_res[n-1] as i64 + black_res[n-2] as i64) { black_res.push(black_val as i32); if black_bt(black_s, i + 1, black_res) { return true; } black_res.pop(); } } false } black_bt(&black_num, 0, &mut black_res); black_res } }
```