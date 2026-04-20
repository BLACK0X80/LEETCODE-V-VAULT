# Count Number of Nice Subarrays

**Difficulty:** Medium
**Tags:** Array, Hash Table, Math, Sliding Window, Prefix Sum

---

## Problem

<p>Given an array of integers <code>nums</code> and an integer <code>k</code>. A continuous subarray is called <strong>nice</strong> if there are <code>k</code> odd numbers on it.</p>

<p>Return <em>the number of <strong>nice</strong> sub-arrays</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,2,1,1], k = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> The only sub-arrays with 3 odd numbers are [1,1,2,1] and [1,2,1,1].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,4,6], k = 1
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no odd numbers in the array.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,2,2,1,2,2,1,2,2,2], k = 2
<strong>Output:</strong> 16
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 50000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10^5</code></li>
	<li><code>1 &lt;= k &lt;= nums.length</code></li>
</ul>


## Hints

1. After replacing each even by zero and every odd by one can we use prefix sum to find answer ?
2. Can we use two pointers to count number of sub-arrays ?
3. Can we store the indices of odd numbers and for each k indices count the number of sub-arrays that contains them ?

## Solution

```rust
impl Solution { pub fn number_of_subarrays(black_n: Vec<i32>, black_k: i32) -> i32 { let (mut black_res, mut black_cur, mut black_l1, mut black_l2) = (0, 0, 0, 0); for black_r in 0..black_n.len() { if black_n[black_r] & 1 == 1 { black_cur += 1; } while black_cur > black_k { if black_n[black_l1] & 1 == 1 { black_cur -= 1; } black_l1 += 1; black_l2 = black_l1; } if black_cur == black_k { while black_n[black_l2] & 1 == 0 { black_l2 += 1; } black_res += (black_l2 - black_l1 + 1) as i32; } } black_res } }
```