# Array With Elements Not Equal to Average of Neighbors

**Difficulty:** Medium
**Tags:** Array, Greedy, Sorting

---

## Problem

<p>You are given a <strong>0-indexed</strong> array <code>nums</code> of <strong>distinct</strong> integers. You want to rearrange the elements in the array such that every element in the rearranged array is <strong>not</strong> equal to the <strong>average</strong> of its neighbors.</p>

<p>More formally, the rearranged array should have the property such that for every <code>i</code> in the range <code>1 &lt;= i &lt; nums.length - 1</code>, <code>(nums[i-1] + nums[i+1]) / 2</code> is <strong>not</strong> equal to <code>nums[i]</code>.</p>

<p>Return <em><strong>any</strong> rearrangement of </em><code>nums</code><em> that meets the requirements</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4,5]
<strong>Output:</strong> [1,2,4,5,3]
<strong>Explanation:</strong>
When i=1, nums[i] = 2, and the average of its neighbors is (1+4) / 2 = 2.5.
When i=2, nums[i] = 4, and the average of its neighbors is (2+5) / 2 = 3.5.
When i=3, nums[i] = 5, and the average of its neighbors is (4+3) / 2 = 3.5.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [6,2,0,9,7]
<strong>Output:</strong> [9,7,6,2,0]
<strong>Explanation:</strong>
When i=1, nums[i] = 7, and the average of its neighbors is (9+6) / 2 = 7.5.
When i=2, nums[i] = 6, and the average of its neighbors is (7+2) / 2 = 4.5.
When i=3, nums[i] = 2, and the average of its neighbors is (6+0) / 2 = 3.
Note that the original array [6,2,0,9,7] also satisfies the conditions.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. A number can be the average of its neighbors if one neighbor is smaller than the number and the other is greater than the number.
2. We can put numbers smaller than the median on odd indices and the rest on even indices.

## Solution

```rust
impl Solution { pub fn rearrange_array(mut black_n: Vec<i32>) -> Vec<i32> { black_n.sort_unstable(); let mut black_res = vec![0; black_n.len()]; let (mut black_l, mut black_r) = (0, (black_n.len() + 1) / 2); for black_i in 0..black_n.len() { if black_i % 2 == 0 { black_res[black_i] = black_n[black_l]; black_l += 1; } else { black_res[black_i] = black_n[black_r]; black_r += 1; } } black_res } }
```