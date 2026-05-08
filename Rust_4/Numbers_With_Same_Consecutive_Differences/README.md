# Numbers With Same Consecutive Differences

**Difficulty:** Medium
**Tags:** Backtracking, Breadth-First Search

---

## Problem

<p>Given two integers n and k, return <em>an array of all the integers of length </em><code>n</code><em> where the difference between every two consecutive digits is </em><code>k</code>. You may return the answer in <strong>any order</strong>.</p>

<p>Note that the integers should not have leading zeros. Integers as <code>02</code> and <code>043</code> are not allowed.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 3, k = 7
<strong>Output:</strong> [181,292,707,818,929]
<strong>Explanation:</strong> Note that 070 is not a valid number, because it has leading zeroes.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 2, k = 1
<strong>Output:</strong> [10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 9</code></li>
	<li><code>0 &lt;= k &lt;= 9</code></li>
</ul>



## Solution

```rust
impl Solution { pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> { let mut black_q: std::collections::VecDeque<i32> = (1..10).collect(); for _ in 1..n { for _ in 0..black_q.len() { let black_val = black_q.pop_front().unwrap(); let black_last = black_val % 10; if k == 0 { black_q.push_back(black_val * 10 + black_last); } else { for &black_next in &[black_last + k, black_last - k] { if black_next >= 0 && black_next <= 9 { black_q.push_back(black_val * 10 + black_next); } } } } } black_q.into_iter().collect() } }
```