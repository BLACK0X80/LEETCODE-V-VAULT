# Find the Longest Equal Subarray

**Difficulty:** Medium
**Tags:** Array, Hash Table, Binary Search, Sliding Window

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code> and an integer <code>k</code>.</p>

<p>A subarray is called <strong>equal</strong> if all of its elements are equal. Note that the empty subarray is an <strong>equal</strong> subarray.</p>

<p>Return <em>the length of the <strong>longest</strong> possible equal subarray after deleting <strong>at most</strong> </em><code>k</code><em> elements from </em><code>nums</code>.</p>

<p>A <b>subarray</b> is a contiguous, possibly empty sequence of elements within an array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,3,2,3,1,3], k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> It&#39;s optimal to delete the elements at index 2 and index 4.
After deleting them, nums becomes equal to [1, 3, 3, 3].
The longest equal subarray starts at i = 1 and ends at j = 3 with length equal to 3.
It can be proven that no longer equal subarrays can be created.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,2,2,1,1], k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> It&#39;s optimal to delete the elements at index 2 and index 3.
After deleting them, nums becomes equal to [1, 1, 1, 1].
The array itself is an equal subarray, so the answer is 4.
It can be proven that no longer equal subarrays can be created.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= nums.length</code></li>
	<li><code>0 &lt;= k &lt;= nums.length</code></li>
</ul>


## Hints

1. <div class="_1l1MA">For each number <code>x</code> in <code>nums</code>, create a sorted list <code>indices<sub>x</sub></code> of all indices <code>i</code> such that <code>nums[i] == x</code>.</div>
2. <div class="_1l1MA">On every <code>indices<sub>x</sub></code>, execute a sliding window technique.</div>
3. <div class="_1l1MA">For each <code>indices<sub>x</sub></code>, find <code>i, j</code> such that <code>(indices<sub>x</sub>[j] - indices<sub>x</sub>[i]) - (j - i) <= k</code> and <code>j - i + 1</code> is maximized.</div>
4. <div class="_1l1MA">The answer would be the maximum of <code>j - i + 1</code> for all <code>indices<sub>x</sub></code>.</div>

## Solution

```rust
impl Solution { pub fn longest_equal_subarray(black_n: Vec<i32>, black_k: i32) -> i32 { let (mut black_m, mut black_i, mut black_max_f) = (std::collections::HashMap::new(), 0, 0); for black_j in 0..black_n.len() { let black_c = black_m.entry(black_n[black_j]).or_insert(0); *black_c += 1; black_max_f = black_max_f.max(*black_c); if (black_j - black_i + 1) as i32 - black_max_f > black_k { *black_m.get_mut(&black_n[black_i]).unwrap() -= 1; black_i += 1; } } black_max_f } }
```