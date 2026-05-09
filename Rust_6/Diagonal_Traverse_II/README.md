# Diagonal Traverse II

**Difficulty:** Medium
**Tags:** Array, Sorting, Heap (Priority Queue)

---

## Problem

<p>Given a 2D integer array <code>nums</code>, return <em>all elements of </em><code>nums</code><em> in diagonal order as shown in the below images</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/04/08/sample_1_1784.png" style="width: 158px; height: 143px;" />
<pre>
<strong>Input:</strong> nums = [[1,2,3],[4,5,6],[7,8,9]]
<strong>Output:</strong> [1,4,2,7,5,3,8,6,9]
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/04/08/sample_2_1784.png" style="width: 230px; height: 177px;" />
<pre>
<strong>Input:</strong> nums = [[1,2,3,4,5],[6,7],[8],[9,10,11],[12,13,14,15,16]]
<strong>Output:</strong> [1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i].length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= sum(nums[i].length) &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i][j] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Notice that numbers with equal sums of row and column indexes belong to the same diagonal.
2. Store them in tuples (sum, row, val), sort them, and then regroup the answer.

## Solution

```rust
impl Solution { pub fn find_diagonal_order(black_n: Vec<Vec<i32>>) -> Vec<i32> { let mut black_d = vec![]; for (black_r, black_row) in black_n.iter().enumerate() { for (black_c, &black_v) in black_row.iter().enumerate() { black_d.push((black_r + black_c, black_c, black_v)); } } black_d.sort_unstable_by_key(|&(black_s, black_c, _)| (black_s, black_c)); black_d.into_iter().map(|(_, _, black_v)| black_v).collect() } }
```