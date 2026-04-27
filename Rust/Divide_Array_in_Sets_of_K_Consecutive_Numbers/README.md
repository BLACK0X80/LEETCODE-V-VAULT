# Divide Array in Sets of K Consecutive Numbers

**Difficulty:** Medium
**Tags:** Array, Hash Table, Greedy, Sorting

---

## Problem

<p>Given an array of integers <code>nums</code> and a positive integer <code>k</code>, check whether it is possible to divide this array into sets of <code>k</code> consecutive numbers.</p>

<p>Return <code>true</code> <em>if it is possible</em>.<strong> </strong>Otherwise, return <code>false</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,3,4,4,5,6], k = 4
<strong>Output:</strong> true
<strong>Explanation:</strong> Array can be divided into [1,2,3,4] and [3,4,5,6].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,2,1,2,3,4,3,4,5,9,10,11], k = 3
<strong>Output:</strong> true
<strong>Explanation:</strong> Array can be divided into [1,2,3] , [2,3,4] , [3,4,5] and [9,10,11].
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4], k = 3
<strong>Output:</strong> false
<strong>Explanation:</strong> Each array should be divided in subarrays of size 3.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>

<p>&nbsp;</p>
<strong>Note:</strong> This question is the same as&nbsp;846:&nbsp;<a href="https://leetcode.com/problems/hand-of-straights/" target="_blank">https://leetcode.com/problems/hand-of-straights/</a>

## Hints

1. If the smallest number in the possible-to-split array is V, then numbers V+1, V+2, ... V+k-1 must contain there as well.
2. You can iteratively find k sets and remove them from array until it becomes empty.
3. Failure to do so would mean that array is unsplittable.

## Solution

```rust
impl Solution { pub fn is_possible_divide(mut black_nums: Vec<i32>, black_k: i32) -> bool { if black_nums.len() % black_k as usize != 0 { return false; } let mut black_m = std::collections::BTreeMap::new(); for black_n in black_nums { *black_m.entry(black_n).or_insert(0) += 1; } while let Some((&black_start, _)) = black_m.iter().next() { for black_i in 0..black_k { let black_val = black_start + black_i; if let Some(black_count) = black_m.get_mut(&black_val) { *black_count -= 1; if *black_count == 0 { black_m.remove(&black_val); } } else { return false; } } } true } }
```