# Merge Adjacent Equal Elements

**Difficulty:** Medium
**Tags:** Array, Stack, Simulation

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>You must <strong>repeatedly</strong> apply the following merge operation until no more changes can be made:</p>

<ul>
	<li>If any <strong>two adjacent elements are equal</strong>, choose the <strong>leftmost</strong> such adjacent pair in the current array and replace them with a single element equal to their <strong>sum</strong>.</li>
</ul>

<p>After each merge operation, the array size <strong>decreases</strong> by 1. Repeat the process on the updated array until no more changes can be made.</p>

<p>Return the final array after all possible merge operations.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,1,1,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">[3,4]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The middle two elements are equal and merged into <code>1 + 1 = 2</code>, resulting in <code>[3, 2, 2]</code>.</li>
	<li>The last two elements are equal and merged into <code>2 + 2 = 4</code>, resulting in <code>[3, 4]</code>.</li>
	<li>No adjacent equal elements remain. Thus, the answer is <code>[3, 4]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,2,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">[8]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The first two elements are equal and merged into <code>2 + 2 = 4</code>, resulting in <code>[4, 4]</code>.</li>
	<li>The first two elements are equal and merged into <code>4 + 4 = 8</code>, resulting in <code>[8]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,7,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">[3,7,5]</span></p>

<p><strong>Explanation:</strong></p>

<p>There are no adjacent equal elements in the array, so no operations are performed.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code>​​​​​​​</li>
</ul>


## Hints

1. Process the array from left to right using a stack-like structure
2. When the current value equals the top of the stack, merge them by replacing with their sum and continue checking
3. The remaining stack elements form the final array

## Solution

```rust
impl Solution { pub fn merge_adjacent(black_nums: Vec<i32>) -> Vec<i64> { black_nums.into_iter().fold(Vec::new(), |mut black_s, black_x| { let mut black_v = black_x as i64; while let Some(&black_top) = black_s.last() { if black_top == black_v { black_v += black_s.pop().unwrap(); } else { break; } } black_s.push(black_v); black_s }) } }
```