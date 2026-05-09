# Maximum Strength of a Group

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Backtracking, Greedy, Bit Manipulation, Sorting, Enumeration

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code> representing the score of students in an exam. The teacher would like to form one <strong>non-empty</strong> group of students with maximal <strong>strength</strong>, where the strength of a group of students of indices <code>i<sub>0</sub></code>, <code>i<sub>1</sub></code>, <code>i<sub>2</sub></code>, ... , <code>i<sub>k</sub></code> is defined as <code>nums[i<sub>0</sub>] * nums[i<sub>1</sub>] * nums[i<sub>2</sub>] * ... * nums[i<sub>k</sub>​]</code>.</p>

<p>Return <em>the maximum strength of a group the teacher can create</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,-1,-5,2,5,-9]
<strong>Output:</strong> 1350
<strong>Explanation:</strong> One way to form a group of maximal strength is to group the students at indices [0,2,3,4,5]. Their strength is 3 * (-5) * 2 * 5 * (-9) = 1350, which we can show is optimal.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [-4,-5,-4]
<strong>Output:</strong> 20
<strong>Explanation:</strong> Group the students at indices [0, 1] . Then, we&rsquo;ll have a resulting strength of 20. We cannot achieve greater strength.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 13</code></li>
	<li><code>-9 &lt;= nums[i] &lt;= 9</code></li>
</ul>


## Hints

1. Try to generate all pairs of subsets and check which group provides maximal strength.
2. It can also be solved in O(NlogN) by sorting the array and using all positive integers.
3. Use negative integers only in pairs such that their product becomes positive.

## Solution

```rust
impl Solution { pub fn max_strength(nums: Vec<i32>) -> i64 { if nums.len() == 1 { return nums[0] as i64; } let (mut black_max, mut black_min) = (nums[0] as i64, nums[0] as i64); for i in 1..nums.len() { let (black_x, black_tmp) = (nums[i] as i64, black_max); black_max = black_max.max(black_x).max(black_tmp * black_x).max(black_min * black_x); black_min = black_min.min(black_x).min(black_tmp * black_x).min(black_min * black_x); } black_max } }
```