# Sum of Distances

**Difficulty:** Medium
**Tags:** Array, Hash Table, Prefix Sum

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code>. There exists an array <code>arr</code> of length <code>nums.length</code>, where <code>arr[i]</code> is the sum of <code>|i - j|</code> over all <code>j</code> such that <code>nums[j] == nums[i]</code> and <code>j != i</code>. If there is no such <code>j</code>, set <code>arr[i]</code> to be <code>0</code>.</p>

<p>Return <em>the array </em><code>arr</code><em>.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,3,1,1,2]
<strong>Output:</strong> [5,0,3,4,0]
<strong>Explanation:</strong> 
When i = 0, nums[0] == nums[2] and nums[0] == nums[3]. Therefore, arr[0] = |0 - 2| + |0 - 3| = 5. 
When i = 1, arr[1] = 0 because there is no other index with value 3.
When i = 2, nums[2] == nums[0] and nums[2] == nums[3]. Therefore, arr[2] = |2 - 0| + |2 - 3| = 3. 
When i = 3, nums[3] == nums[0] and nums[3] == nums[2]. Therefore, arr[3] = |3 - 0| + |3 - 2| = 4. 
When i = 4, arr[4] = 0 because there is no other index with value 2. 

</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [0,5,3]
<strong>Output:</strong> [0,0,0]
<strong>Explanation:</strong> Since each element in nums is distinct, arr[i] = 0 for all i.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>

<p>&nbsp;</p>
<p><strong>Note:</strong> This question is the same as <a href="https://leetcode.com/problems/intervals-between-identical-elements/description/" target="_blank"> 2121: Intervals Between Identical Elements.</a></p>


## Hints

1. Can we use the prefix sum here?
2. For each number x, collect all the indices where x occurs, and calculate the prefix sum of the array.
3. For each occurrence of x, the indices to the right will be regular subtraction while the indices to the left will be reversed subtraction.

## Solution

```rust
impl Solution { pub fn distance(black_n: Vec<i32>) -> Vec<i64> { let mut black_m = std::collections::HashMap::new(); for (i, &v) in black_n.iter().enumerate() { black_m.entry(v).or_insert(vec![]).push(i as i64); } let mut black_res = vec![0i64; black_n.len()]; for (_, black_v) in black_m { let black_count = black_v.len() as i64; let mut black_total_sum: i64 = black_v.iter().sum(); let mut black_prefix_sum = 0i64; for (i, &black_idx) in black_v.iter().enumerate() { let black_i = i as i64; black_total_sum -= black_idx; black_res[black_idx as usize] = (black_idx * black_i - black_prefix_sum) + (black_total_sum - black_idx * (black_count - black_i - 1)); black_prefix_sum += black_idx; } } black_res } }
```