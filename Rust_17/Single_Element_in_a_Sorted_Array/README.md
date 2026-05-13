# Single Element in a Sorted Array

**Difficulty:** Medium
**Tags:** Array, Binary Search

---

## Problem

<p>You are given a sorted array consisting of only integers where every element appears exactly twice, except for one element which appears exactly once.</p>

<p>Return <em>the single element that appears only once</em>.</p>

<p>Your solution must run in <code>O(log n)</code> time and <code>O(1)</code> space.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> nums = [1,1,2,3,3,4,4,8,8]
<strong>Output:</strong> 2
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> nums = [3,3,7,7,10,11,11]
<strong>Output:</strong> 10
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn single_non_duplicate(black_n: Vec<i32>) -> i32 { let (mut black_l, mut black_r) = (0, black_n.len() - 1); while black_l < black_r { let mut black_m = black_l + (black_r - black_l) / 2; if black_m % 2 == 1 { black_m -= 1; } if black_n[black_m] == black_n[black_m + 1] { black_l = black_m + 2; } else { black_r = black_m; } } black_n[black_l] } }
```