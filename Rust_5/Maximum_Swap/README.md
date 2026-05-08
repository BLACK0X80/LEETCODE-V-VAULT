# Maximum Swap

**Difficulty:** Medium
**Tags:** Math, Greedy

---

## Problem

<p>You are given an integer <code>num</code>. You can swap two digits at most once to get the maximum valued number.</p>

<p>Return <em>the maximum valued number you can get</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num = 2736
<strong>Output:</strong> 7236
<strong>Explanation:</strong> Swap the number 2 and the number 7.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num = 9973
<strong>Output:</strong> 9973
<strong>Explanation:</strong> No swap.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= num &lt;= 10<sup>8</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn maximum_swap(black_num: i32) -> i32 { let mut black_s = black_num.to_string().into_bytes(); let mut black_last = [0; 10]; for i in 0..black_s.len() { black_last[(black_s[i] - b'0') as usize] = i; } for i in 0..black_s.len() { for d in ( (black_s[i] - b'0' + 1) as usize..10).rev() { if black_last[d] > i { black_s.swap(i, black_last[d]); return String::from_utf8(black_s).unwrap().parse().unwrap(); } } } black_num } }
```