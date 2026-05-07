# Removing Minimum and Maximum From Array

**Difficulty:** Medium
**Tags:** Array, Greedy

---

## Problem

<p>You are given a <strong>0-indexed</strong> array of <strong>distinct</strong> integers <code>nums</code>.</p>

<p>There is an element in <code>nums</code> that has the <strong>lowest</strong> value and an element that has the <strong>highest</strong> value. We call them the <strong>minimum</strong> and <strong>maximum</strong> respectively. Your goal is to remove <strong>both</strong> these elements from the array.</p>

<p>A <strong>deletion</strong> is defined as either removing an element from the <strong>front</strong> of the array or removing an element from the <strong>back</strong> of the array.</p>

<p>Return <em>the <strong>minimum</strong> number of deletions it would take to remove <strong>both</strong> the minimum and maximum element from the array.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,<u><strong>10</strong></u>,7,5,4,<u><strong>1</strong></u>,8,6]
<strong>Output:</strong> 5
<strong>Explanation:</strong> 
The minimum element in the array is nums[5], which is 1.
The maximum element in the array is nums[1], which is 10.
We can remove both the minimum and maximum by removing 2 elements from the front and 3 elements from the back.
This results in 2 + 3 = 5 deletions, which is the minimum number possible.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [0,<u><strong>-4</strong></u>,<u><strong>19</strong></u>,1,8,-2,-3,5]
<strong>Output:</strong> 3
<strong>Explanation:</strong> 
The minimum element in the array is nums[1], which is -4.
The maximum element in the array is nums[2], which is 19.
We can remove both the minimum and maximum by removing 3 elements from the front.
This results in only 3 deletions, which is the minimum number possible.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [<u><strong>101</strong></u>]
<strong>Output:</strong> 1
<strong>Explanation:</strong>  
There is only one element in the array, which makes it both the minimum and maximum element.
We can remove it with 1 deletion.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>5</sup> &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li>The integers in <code>nums</code> are <strong>distinct</strong>.</li>
</ul>


## Hints

1. There can only be three scenarios for deletions such that both minimum and maximum elements are removed:
2. Scenario 1: Both elements are removed by only deleting from the front.
3. Scenario 2: Both elements are removed by only deleting from the back.
4. Scenario 3: Delete from the front to remove one of the elements, and delete from the back to remove the other element.
5. Compare which of the three scenarios results in the minimum number of moves.

## Solution

```rust
impl Solution { pub fn minimum_deletions(nums: Vec<i32>) -> i32 { let (black_n, mut black_min_idx, mut black_max_idx) = (nums.len() as i32, 0, 0); for black_i in 0..black_n { if nums[black_i as usize] < nums[black_min_idx as usize] { black_min_idx = black_i; } if nums[black_i as usize] > nums[black_max_idx as usize] { black_max_idx = black_i; } } let (black_a, black_b) = (black_min_idx.min(black_max_idx), black_min_idx.max(black_max_idx)); (black_b + 1).min(black_n - black_a).min(black_a + 1 + black_n - black_b) } }
```