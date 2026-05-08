# Minimum Operations to Make Array Equal to Target

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Stack, Greedy, Monotonic Stack

---

## Problem

<p>You are given two positive integer arrays <code>nums</code> and <code>target</code>, of the same length.</p>

<p>In a single operation, you can select any subarray of <code>nums</code> and increment each element within that subarray by 1 or decrement each element within that subarray by 1.</p>

<p>Return the <strong>minimum</strong> number of operations required to make <code>nums</code> equal to the array <code>target</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,5,1,2], target = [4,6,2,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>We will perform the following operations to make <code>nums</code> equal to <code>target</code>:<br />
- Increment&nbsp;<code>nums[0..3]</code> by 1, <code>nums = [4,6,2,3]</code>.<br />
- Increment&nbsp;<code>nums[3..3]</code> by 1, <code>nums = [4,6,2,4]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,3,2], target = [2,1,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p>We will perform the following operations to make <code>nums</code> equal to <code>target</code>:<br />
- Increment&nbsp;<code>nums[0..0]</code> by 1, <code>nums = [2,3,2]</code>.<br />
- Decrement&nbsp;<code>nums[1..1]</code> by 1, <code>nums = [2,2,2]</code>.<br />
- Decrement&nbsp;<code>nums[1..1]</code> by 1, <code>nums = [2,1,2]</code>.<br />
- Increment&nbsp;<code>nums[2..2]</code> by 1, <code>nums = [2,1,3]</code>.<br />
- Increment&nbsp;<code>nums[2..2]</code> by 1, <code>nums = [2,1,4]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length == target.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i], target[i] &lt;= 10<sup>8</sup></code></li>
</ul>


## Hints

1. Change <code>nums'[i] = nums[i] - target[i]</code>, so our goal is to make <code>nums'</code> into all 0s.
2. Divide and conquer.

## Solution

```rust
impl Solution {
    pub fn minimum_operations(black_nums: Vec<i32>, black_target: Vec<i32>) -> i64 {
        let mut black_res = 0i64;
        let mut black_prev = 0i64;
        for black_i in 0..black_nums.len() {
            let black_diff = (black_target[black_i] as i64) - (black_nums[black_i] as i64);
            if (black_diff > 0 && black_prev < 0) || (black_diff < 0 && black_prev > 0) {
                black_res += black_diff.abs();
            } else if black_diff.abs() > black_prev.abs() {
                black_res += (black_diff - black_prev).abs();
            }
            black_prev = black_diff;
        }
        black_res
    }
}
```