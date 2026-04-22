# Lexicographical Numbers

**Difficulty:** Medium
**Tags:** Depth-First Search, Trie

---

## Problem

<p>Given an integer <code>n</code>, return all the numbers in the range <code>[1, n]</code> sorted in lexicographical order.</p>

<p>You must write an algorithm that runs in&nbsp;<code>O(n)</code>&nbsp;time and uses <code>O(1)</code> extra space.&nbsp;</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> n = 13
<strong>Output:</strong> [1,10,11,12,13,2,3,4,5,6,7,8,9]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> n = 2
<strong>Output:</strong> [1,2]
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 5 * 10<sup>4</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn lexical_order(black_n: i32) -> Vec<i32> { let (mut black_res, mut black_curr) = (Vec::with_capacity(black_n as usize), 1); for _ in 0..black_n { black_res.push(black_curr); if black_curr * 10 <= black_n { black_curr *= 10; } else { while black_curr % 10 == 9 || black_curr + 1 > black_n { black_curr /= 10; } black_curr += 1; } } black_res } }
```