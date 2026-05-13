# Minimum Operations to Convert All Elements to Zero

**Difficulty:** Medium
**Tags:** Array, Hash Table, Stack, Greedy, Monotonic Stack

---

## Problem

<p>You are given an array <code>nums</code> of size <code>n</code>, consisting of <strong>non-negative</strong> integers. Your task is to apply some (possibly zero) operations on the array so that <strong>all</strong> elements become 0.</p>

<p>In one operation, you can select a <span data-keyword="subarray">subarray</span> <code>[i, j]</code> (where <code>0 &lt;= i &lt;= j &lt; n</code>) and set all occurrences of the <strong>minimum</strong> <strong>non-negative</strong> integer in that subarray to 0.</p>

<p>Return the <strong>minimum</strong> number of operations required to make all elements in the array 0.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [0,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Select the subarray <code>[1,1]</code> (which is <code>[2]</code>), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in <code>[0,0]</code>.</li>
	<li>Thus, the minimum number of operations required is 1.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,1,2,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Select subarray <code>[1,3]</code> (which is <code>[1,2,1]</code>), where the minimum non-negative integer is 1. Setting all occurrences of 1 to 0 results in <code>[3,0,2,0]</code>.</li>
	<li>Select subarray <code>[2,2]</code> (which is <code>[2]</code>), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in <code>[3,0,0,0]</code>.</li>
	<li>Select subarray <code>[0,0]</code> (which is <code>[3]</code>), where the minimum non-negative integer is 3. Setting all occurrences of 3 to 0 results in <code>[0,0,0,0]</code>.</li>
	<li>Thus, the minimum number of operations required is 3.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,1,2,1,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Select subarray <code>[0,5]</code> (which is <code>[1,2,1,2,1,2]</code>), where the minimum non-negative integer is 1. Setting all occurrences of 1 to 0 results in <code>[0,2,0,2,0,2]</code>.</li>
	<li>Select subarray <code>[1,1]</code> (which is <code>[2]</code>), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in <code>[0,0,0,2,0,2]</code>.</li>
	<li>Select subarray <code>[3,3]</code> (which is <code>[2]</code>), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in <code>[0,0,0,0,0,2]</code>.</li>
	<li>Select subarray <code>[5,5]</code> (which is <code>[2]</code>), where the minimum non-negative integer is 2. Setting all occurrences of 2 to 0 results in <code>[0,0,0,0,0,0]</code>.</li>
	<li>Thus, the minimum number of operations required is 4.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Process the values in nums from smallest to largest (excluding 0).
2. For each target value v, identify its maximal contiguous segments (subarrays where nums[i] == v); each segment can be zeroed out in one operation.
3. After setting those segments to zero, dynamically update the remaining array and repeat with the next value.

## Solution

```rust
impl Solution { pub fn min_operations(black_nums: Vec<i32>) -> i32 { let mut black_stk = vec![0]; let mut black_ans = 0; for black_x in black_nums { while *black_stk.last().unwrap() > black_x { black_stk.pop(); } if *black_stk.last().unwrap() < black_x { black_ans += 1; black_stk.push(black_x); } } black_ans } }
```