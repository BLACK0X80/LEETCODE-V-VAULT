# Minimum Array Length After Pair Removals

**Difficulty:** Medium
**Tags:** Array, Hash Table, Two Pointers, Binary Search, Greedy, Counting

---

## Problem

<p>Given an integer array <code>num</code> sorted in non-decreasing order.</p>

<p>You can perform the following operation any number of times:</p>

<ul>
	<li>Choose <strong>two</strong> indices, <code>i</code> and <code>j</code>, where <code>nums[i] &lt; nums[j]</code>.</li>
	<li>Then, remove the elements at indices <code>i</code> and <code>j</code> from <code>nums</code>. The remaining elements retain their original order, and the array is re-indexed.</li>
</ul>

<p>Return the <strong>minimum</strong> length of <code>nums</code> after applying the operation zero or more times.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2024/05/18/tcase1.gif" style="width: 160px; height: 70px;" /></p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1,2,2,3,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2024/05/19/tcase2.gif" style="width: 240px; height: 70px;" /></p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1000000000,1000000000]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>Since both numbers are equal, they cannot be removed.</p>
</div>

<p><strong class="example">Example 4:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,3,4,4,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2024/05/19/tcase3.gif" style="width: 210px; height: 70px;" /></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>nums</code> is sorted in <strong>non-decreasing</strong> order.</li>
</ul>


## Hints

1. To minimize the length of the array, we should maximize the number of operations performed.
2. To perform <code>k</code> operations, it is optimal to use the smallest <code>k</code> values and the largest <code>k</code> values in <code>nums</code>.
3. What is the best way to make pairs from the smallest <code>k</code> values and the largest <code>k</code> values so it is possible to remove all the pairs?
4. If we consider the smallest <code>k</code> values and the largest <code>k</code> values as two separate <strong>sorted 0-indexed</strong> arrays, <code>a</code> and <code>b</code>, It is optimal to pair <code>a[i]</code> and <code>b[i]</code>. So, a <code>k</code> is valid if <code>a[i] < b[i]</code> for all <code>i</code> in the range <code>[0, k - 1]</code>.
5. The greatest possible valid <code>k</code> can be found using binary search.
6. The answer is <code>nums.length - 2 * k</code>.

## Solution

```rust
use std::collections::HashMap; impl Solution { pub fn min_length_after_removals(black_nums: Vec<i32>) -> i32 { let black_n = black_nums.len() as i32; let mut black_map = HashMap::new(); let mut black_max_f = 0; for black_x in black_nums { let black_count = black_map.entry(black_x).or_insert(0); *black_count += 1; black_max_f = black_max_f.max(*black_count); } if black_max_f * 2 > black_n { black_max_f * 2 - black_n } else { black_n % 2 } } }
```