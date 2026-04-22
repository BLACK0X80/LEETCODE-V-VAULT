# Reordered Power of 2

**Difficulty:** Medium
**Tags:** Hash Table, Math, Sorting, Counting, Enumeration

---

## Problem

<p>You are given an integer <code>n</code>. We reorder the digits in any order (including the original order) such that the leading digit is not zero.</p>

<p>Return <code>true</code> <em>if and only if we can do this so that the resulting number is a power of two</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> true
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> false
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>9</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn reordered_power_of2(n: i32) -> bool { let black_count = |mut x: i32| { let mut res = [0; 10]; while x > 0 { res[(x % 10) as usize] += 1; x /= 10; } res }; let black_target = black_count(n); (0..31).any(|i| black_count(1 << i) == black_target) } }
```