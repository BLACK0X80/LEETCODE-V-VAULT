# Minimum Operations to Make All Array Elements Equal

**Difficulty:** Medium
**Tags:** Array, Binary Search, Sorting, Prefix Sum

---

## Problem

<p>You are given an array <code>nums</code> consisting of positive integers.</p>

<p>You are also given an integer array <code>queries</code> of size <code>m</code>. For the <code>i<sup>th</sup></code> query, you want to make all of the elements of <code>nums</code> equal to<code> queries[i]</code>. You can perform the following operation on the array <strong>any</strong> number of times:</p>

<ul>
	<li><strong>Increase</strong> or <strong>decrease</strong> an element of the array by <code>1</code>.</li>
</ul>

<p>Return <em>an array </em><code>answer</code><em> of size </em><code>m</code><em> where </em><code>answer[i]</code><em> is the <strong>minimum</strong> number of operations to make all elements of </em><code>nums</code><em> equal to </em><code>queries[i]</code>.</p>

<p><strong>Note</strong> that after each query the array is reset to its original state.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,1,6,8], queries = [1,5]
<strong>Output:</strong> [14,10]
<strong>Explanation:</strong> For the first query we can do the following operations:
- Decrease nums[0] 2 times, so that nums = [1,1,6,8].
- Decrease nums[2] 5 times, so that nums = [1,1,1,8].
- Decrease nums[3] 7 times, so that nums = [1,1,1,1].
So the total number of operations for the first query is 2 + 5 + 7 = 14.
For the second query we can do the following operations:
- Increase nums[0] 2 times, so that nums = [5,1,6,8].
- Increase nums[1] 4 times, so that nums = [5,5,6,8].
- Decrease nums[2] 1 time, so that nums = [5,5,5,8].
- Decrease nums[3] 3 times, so that nums = [5,5,5,5].
So the total number of operations for the second query is 2 + 4 + 1 + 3 = 10.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,9,6,3], queries = [10]
<strong>Output:</strong> [20]
<strong>Explanation:</strong> We can increase each value in the array to 10. The total number of operations will be 8 + 1 + 4 + 7 = 20.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums.length</code></li>
	<li><code>m == queries.length</code></li>
	<li><code>1 &lt;= n, m &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i], queries[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. For each query, you should decrease all elements greater than queries[i] and increase all elements less than queries[i].
2. The answer is the sum of absolute differences between queries[i] and every element of the array. How do you calculate that optimally?

## Solution

```rust
impl Solution { pub fn min_operations(mut black_n: Vec<i32>, q: Vec<i32>) -> Vec<i64> { black_n.sort_unstable(); let black_sz = black_n.len(); let mut black_pre = vec![0i64; black_sz + 1]; (0..black_sz).for_each(|black_i| black_pre[black_i+1] = black_pre[black_i] + black_n[black_i] as i64); q.into_iter().map(|black_v| { let black_idx = black_n.partition_point(|&black_x| black_x < black_v); (black_v as i64 * black_idx as i64 - black_pre[black_idx]) + (black_pre[black_sz] - black_pre[black_idx] - black_v as i64 * (black_sz - black_idx) as i64) }).collect() } }
```