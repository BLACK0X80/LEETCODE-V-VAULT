# Contiguous Array

**Difficulty:** Medium
**Tags:** Array, Hash Table, Prefix Sum

---

## Problem

<p>Given a binary array <code>nums</code>, return <em>the maximum length of a contiguous subarray with an equal number of </em><code>0</code><em> and </em><code>1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [0,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> [0, 1] is the longest contiguous subarray with an equal number of 0 and 1.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [0,1,0]
<strong>Output:</strong> 2
<strong>Explanation:</strong> [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [0,1,1,1,1,1,0,0,0]
<strong>Output:</strong> 6
<strong>Explanation:</strong> [1,1,1,0,0,0] is the longest contiguous subarray with equal number of 0 and 1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>nums[i]</code> is either <code>0</code> or <code>1</code>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn find_max_length(black_n: Vec<i32>) -> i32 { let mut black_m = std::collections::HashMap::new(); black_m.insert(0, -1); let (mut black_s, mut black_res) = (0, 0); for (i, &v) in black_n.iter().enumerate() { black_s += if v == 1 { 1 } else { -1 }; if let Some(&p) = black_m.get(&black_s) { black_res = black_res.max(i as i32 - p); } else { black_m.insert(black_s, i as i32); } } black_res } }
```