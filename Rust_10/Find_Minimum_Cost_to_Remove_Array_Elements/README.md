# Find Minimum Cost to Remove Array Elements

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given an integer array <code>nums</code>. Your task is to remove <strong>all elements</strong> from the array by performing one of the following operations at each step until <code>nums</code> is empty:</p>

<ul>
	<li>Choose any two elements from the first three elements of <code>nums</code> and remove them. The cost of this operation is the <strong>maximum</strong> of the two elements removed.</li>
	<li>If fewer than three elements remain in <code>nums</code>, remove all the remaining elements in a single operation. The cost of this operation is the <strong>maximum</strong> of the remaining elements.</li>
</ul>

<p>Return the <strong>minimum</strong> cost required to remove all the elements.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [6,2,8,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">12</span></p>

<p><strong>Explanation:</strong></p>

<p>Initially, <code>nums = [6, 2, 8, 4]</code>.</p>

<ul>
	<li>In the first operation, remove <code>nums[0] = 6</code> and <code>nums[2] = 8</code> with a cost of <code>max(6, 8) = 8</code>. Now, <code>nums = [2, 4]</code>.</li>
	<li>In the second operation, remove the remaining elements with a cost of <code>max(2, 4) = 4</code>.</li>
</ul>

<p>The cost to remove all elements is <code>8 + 4 = 12</code>. This is the minimum cost to remove all elements in <code>nums</code>. Hence, the output is 12.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,1,3,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p>Initially, <code>nums = [2, 1, 3, 3]</code>.</p>

<ul>
	<li>In the first operation, remove <code>nums[0] = 2</code> and <code>nums[1] = 1</code> with a cost of <code>max(2, 1) = 2</code>. Now, <code>nums = [3, 3]</code>.</li>
	<li>In the second operation remove the remaining elements with a cost of <code>max(3, 3) = 3</code>.</li>
</ul>

<p>The cost to remove all elements is <code>2 + 3 = 5</code>. This is the minimum cost to remove all elements in <code>nums</code>. Hence, the output is 5.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Can we use dynamic programming here?
2. Use dynamic programming. The process guarantees that the remaining elements form a prefix of the array with at most one previous element.
3. Define the state as <code>dp[i][j]</code>, where <code>i</code> represents the last remaining element and <code>j</code> represents the starting index of the current prefix.

## Solution

```rust
impl Solution { pub fn min_cost(nums: Vec<i32>) -> i32 { let n = nums.len(); let mut black_dp = std::collections::HashMap::new(); fn solve(i: usize, last: usize, n: usize, nums: &Vec<i32>, memo: &mut std::collections::HashMap<(usize, usize), i32>) -> i32 { if i >= n { return nums[last]; } if i + 1 == n { return nums[last].max(nums[i]); } if let Some(&res) = memo.get(&(i, last)) { return res; } let op1 = nums[i].max(nums[i+1]) + solve(i+2, last, n, nums, memo); let op2 = nums[last].max(nums[i]) + solve(i+2, i+1, n, nums, memo); let op3 = nums[last].max(nums[i+1]) + solve(i+2, i, n, nums, memo); let black_res = op1.min(op2).min(op3); memo.insert((i, last), black_res); black_res } solve(1, 0, n, &nums, &mut black_dp) } }
```