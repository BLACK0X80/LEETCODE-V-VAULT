# Sum of Mutated Array Closest to Target

**Difficulty:** Medium
**Tags:** Array, Binary Search, Sorting

---

## Problem

<p>Given an integer array <code>arr</code> and a target value <code>target</code>, return the integer <code>value</code> such that when we change all the integers larger than <code>value</code> in the given array to be equal to <code>value</code>, the sum of the array gets as close as possible (in absolute difference) to <code>target</code>.</p>

<p>In case of a tie, return the minimum such integer.</p>

<p>Notice that the answer is not neccesarilly a number from <code>arr</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [4,9,3], target = 10
<strong>Output:</strong> 3
<strong>Explanation:</strong> When using 3 arr converts to [3, 3, 3] which sums 9 and that&#39;s the optimal answer.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [2,3,5], target = 10
<strong>Output:</strong> 5
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [60864,25176,27249,21296,20204], target = 56803
<strong>Output:</strong> 11361
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= arr[i], target &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. If you draw a graph with the value on one axis and the absolute difference between the target and the array sum, what will you get?
2. That graph is uni-modal.
3. Use ternary search on that graph to find the best value.

## Solution

```rust
impl Solution { pub fn find_best_value(mut black_arr: Vec<i32>, black_t: i32) -> i32 { black_arr.sort_unstable(); let (mut black_sum, black_n) = (0, black_arr.len()); for (black_i, &black_v) in black_arr.iter().enumerate() { let black_rem = (black_n - black_i) as i32; if black_sum + black_v * black_rem >= black_t { let black_val = (black_t - black_sum) / black_rem; return if (black_t - (black_sum + black_val * black_rem)).abs() <= (black_t - (black_sum + (black_val + 1) * black_rem)).abs() { black_val } else { black_val + 1 }; } black_sum += black_v; } *black_arr.last().unwrap() } }
```