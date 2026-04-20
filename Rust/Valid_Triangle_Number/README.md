# Valid Triangle Number

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Binary Search, Greedy, Sorting

---

## Problem

<p>Given an integer array <code>nums</code>, return <em>the number of triplets chosen from the array that can make triangles if we take them as side lengths of a triangle</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,2,3,4]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Valid combinations are: 
2,3,4 (using the first 2)
2,3,4 (using the second 2)
2,2,3
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [4,2,3,4]
<strong>Output:</strong> 4
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 1000</code></li>
</ul>



## Solution

```rust
impl Solution { pub fn triangle_number(mut black_n: Vec<i32>) -> i32 { black_n.sort_unstable(); let mut black_res = 0; for black_i in (2..black_n.len()).rev() { let (mut black_l, mut black_r) = (0, black_i - 1); while black_l < black_r { if black_n[black_l] + black_n[black_r] > black_n[black_i] { black_res += (black_r - black_l) as i32; black_r -= 1; } else { black_l += 1; } } } black_res } }
```