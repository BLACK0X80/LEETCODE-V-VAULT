# Ways to Split Array Into Three Subarrays

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Binary Search, Prefix Sum

---

## Problem

<p>A split of an integer array is <strong>good</strong> if:</p>

<ul>
	<li>The array is split into three <strong>non-empty</strong> contiguous subarrays - named <code>left</code>, <code>mid</code>, <code>right</code> respectively from left to right.</li>
	<li>The sum of the elements in <code>left</code> is less than or equal to the sum of the elements in <code>mid</code>, and the sum of the elements in <code>mid</code> is less than or equal to the sum of the elements in <code>right</code>.</li>
</ul>

<p>Given <code>nums</code>, an array of <strong>non-negative</strong> integers, return <em>the number of <strong>good</strong> ways to split</em> <code>nums</code>. As the number may be too large, return it <strong>modulo</strong> <code>10<sup>9 </sup>+ 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only good way to split nums is [1] [1] [1].</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,2,2,5,0]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are three good ways of splitting nums:
[1] [2] [2,2,5,0]
[1] [2,2] [2,5,0]
[1,2] [2,2] [5,0]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,2,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no good way to split nums.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Create a prefix array to efficiently find the sum of subarrays.
2. As we are dividing the array into three subarrays, there are two "walls". Iterate over the right wall positions and find where the left wall could be for each right wall position.
3. Use binary search to find the left-most position and right-most position the left wall could be.

## Solution

```rust
impl Solution { pub fn ways_to_split(black_nums: Vec<i32>) -> i32 { let (black_n, mut black_p) = (black_nums.len(), vec![0; black_nums.len() + 1]); for black_i in 0..black_n { black_p[black_i + 1] = black_p[black_i] + black_nums[black_i]; } let (mut black_ans, mut black_j, mut black_k, black_mod) = (0i64, 0, 0, 1_000_000_007); for black_i in 1..black_n - 1 { black_j = black_j.max(black_i + 1); while black_j < black_n && black_p[black_j] < 2 * black_p[black_i] { black_j += 1; } black_k = black_k.max(black_j); while black_k < black_n && 2 * black_p[black_k] <= black_p[black_i] + black_p[black_n] { black_k += 1; } if black_k > black_j { black_ans = (black_ans + (black_k - black_j) as i64) % black_mod; } } black_ans as i32 } }
```