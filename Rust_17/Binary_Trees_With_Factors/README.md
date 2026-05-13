# Binary Trees With Factors

**Difficulty:** Medium
**Tags:** Array, Hash Table, Dynamic Programming, Sorting

---

## Problem

<p>Given an array of unique integers, <code>arr</code>, where each integer <code>arr[i]</code> is strictly greater than <code>1</code>.</p>

<p>We make a binary tree using these integers, and each number may be used for any number of times. Each non-leaf node&#39;s value should be equal to the product of the values of its children.</p>

<p>Return <em>the number of binary trees we can make</em>. The answer may be too large so return the answer <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [2,4]
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can make these trees: <code>[2], [4], [4, 2, 2]</code></pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [2,4,5,10]
<strong>Output:</strong> 7
<strong>Explanation:</strong> We can make these trees: <code>[2], [4], [5], [10], [4, 2, 2], [10, 2, 5], [10, 5, 2]</code>.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 1000</code></li>
	<li><code>2 &lt;= arr[i] &lt;= 10<sup>9</sup></code></li>
	<li>All the values of <code>arr</code> are <strong>unique</strong>.</li>
</ul>



## Solution

```rust
use std::collections::HashMap; impl Solution { pub fn num_factored_binary_trees(mut black_arr: Vec<i32>) -> i32 { black_arr.sort(); let mut black_map = HashMap::new(); let black_mod = 1_000_000_007; for &black_x in &black_arr { let mut black_count = 1u64; for &black_y in &black_arr { if black_y >= black_x { break; } if black_x % black_y == 0 { if let Some(&black_val) = black_map.get(&(black_x / black_y)) { black_count = (black_count + black_map[&black_y] * black_val) % black_mod; } } } black_map.insert(black_x, black_count); } (black_map.values().sum::<u64>() % black_mod) as i32 } }
```