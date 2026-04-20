# Find All Duplicates in an Array

**Difficulty:** Medium
**Tags:** Array, Hash Table, Sorting

---

## Problem

<p>Given an integer array <code>nums</code> of length <code>n</code> where all the integers of <code>nums</code> are in the range <code>[1, n]</code> and each integer appears <strong>at most</strong> <strong>twice</strong>, return <em>an array of all the integers that appears <strong>twice</strong></em>.</p>

<p>You must write an algorithm that runs in <code>O(n)</code> time and uses only <em>constant</em> auxiliary space, excluding the space needed to store the output</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> nums = [4,3,2,7,8,2,3,1]
<strong>Output:</strong> [2,3]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> nums = [1,1,2]
<strong>Output:</strong> [1]
</pre><p><strong class="example">Example 3:</strong></p>
<pre><strong>Input:</strong> nums = [1]
<strong>Output:</strong> []
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= n</code></li>
	<li>Each element in <code>nums</code> appears <strong>once</strong> or <strong>twice</strong>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn find_duplicates(mut black_n: Vec<i32>) -> Vec<i32> { let mut black_res = vec![]; for black_i in 0..black_n.len() { let black_val = black_n[black_i].abs() as usize; if black_n[black_val-1] < 0 { black_res.push(black_val as i32); } else { black_n[black_val-1] *= -1; } } black_res } }
```