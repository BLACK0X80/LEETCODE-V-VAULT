# Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit

**Difficulty:** Medium
**Tags:** Array, Queue, Sliding Window, Heap (Priority Queue), Ordered Set, Monotonic Queue

---

## Problem

<p>Given an array of integers <code>nums</code> and an integer <code>limit</code>, return the size of the longest <strong>non-empty</strong> subarray such that the absolute difference between any two elements of this subarray is less than or equal to <code>limit</code><em>.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [8,2,4,7], limit = 4
<strong>Output:</strong> 2 
<strong>Explanation:</strong> All subarrays are: 
[8] with maximum absolute diff |8-8| = 0 &lt;= 4.
[8,2] with maximum absolute diff |8-2| = 6 &gt; 4. 
[8,2,4] with maximum absolute diff |8-2| = 6 &gt; 4.
[8,2,4,7] with maximum absolute diff |8-2| = 6 &gt; 4.
[2] with maximum absolute diff |2-2| = 0 &lt;= 4.
[2,4] with maximum absolute diff |2-4| = 2 &lt;= 4.
[2,4,7] with maximum absolute diff |2-7| = 5 &gt; 4.
[4] with maximum absolute diff |4-4| = 0 &lt;= 4.
[4,7] with maximum absolute diff |4-7| = 3 &lt;= 4.
[7] with maximum absolute diff |7-7| = 0 &lt;= 4. 
Therefore, the size of the longest subarray is 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [10,1,2,4,7,2], limit = 5
<strong>Output:</strong> 4 
<strong>Explanation:</strong> The subarray [2,4,7,2] is the longest since the maximum absolute diff is |2-7| = 5 &lt;= 5.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [4,2,2,2,4,4,2,2], limit = 0
<strong>Output:</strong> 3
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= limit &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Use a sliding window approach keeping the maximum and minimum value using a data structure like a multiset from STL in C++.
2. More specifically, use the two pointer technique, moving the right pointer as far as possible to the right until the subarray is not valid (maxValue - minValue > limit), then moving the left pointer until the subarray is valid again (maxValue - minValue <= limit). Keep repeating this process.

## Solution

```rust
impl Solution { pub fn longest_subarray(black_n: Vec<i32>, black_l: i32) -> i32 { let (mut black_min, mut black_max, mut black_i, mut black_res) = (std::collections::VecDeque::new(), std::collections::VecDeque::new(), 0, 0); for black_j in 0..black_n.len() { while !black_min.is_empty() && black_n[*black_min.back().unwrap()] >= black_n[black_j] { black_min.pop_back(); } while !black_max.is_empty() && black_n[*black_max.back().unwrap()] <= black_n[black_j] { black_max.pop_back(); } black_min.push_back(black_j); black_max.push_back(black_j); while black_n[*black_max.front().unwrap()] - black_n[*black_min.front().unwrap()] > black_l { black_i += 1; if *black_min.front().unwrap() < black_i { black_min.pop_front(); } if *black_max.front().unwrap() < black_i { black_max.pop_front(); } } black_res = black_res.max(black_j - black_i + 1); } black_res as i32 } }
```