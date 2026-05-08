# Count the Number of Fair Pairs

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Binary Search, Sorting

---

## Problem

<p>Given a <strong>0-indexed</strong> integer array <code>nums</code> of size <code>n</code> and two integers <code>lower</code> and <code>upper</code>, return <em>the number of fair pairs</em>.</p>

<p>A pair <code>(i, j)</code> is <b>fair </b>if:</p>

<ul>
	<li><code>0 &lt;= i &lt; j &lt; n</code>, and</li>
	<li><code>lower &lt;= nums[i] + nums[j] &lt;= upper</code></li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [0,1,7,4,4,5], lower = 3, upper = 6
<strong>Output:</strong> 6
<strong>Explanation:</strong> There are 6 fair pairs: (0,3), (0,4), (0,5), (1,3), (1,4), and (1,5).
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,7,9,2,5], lower = 11, upper = 11
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is a single fair pair: (2,3).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>nums.length == n</code></li>
	<li><code><font face="monospace">-10<sup>9</sup></font>&nbsp;&lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code><font face="monospace">-10<sup>9</sup>&nbsp;&lt;= lower &lt;= upper &lt;= 10<sup>9</sup></font></code></li>
</ul>


## Hints

1. Sort the array in ascending order.
2. For each number in the array, keep track of the smallest and largest numbers in the array that can form a fair pair with this number.
3. As you move to larger number, both boundaries move down.

## Solution

```rust
impl Solution { pub fn count_fair_pairs(mut black_nums: Vec<i32>, black_l: i32, black_u: i32) -> i64 { black_nums.sort_unstable(); let black_f = |black_t: i32| { let (mut black_c, mut black_i, mut black_j) = (0i64, 0, black_nums.len() - 1); while black_i < black_j { if black_nums[black_i] + black_nums[black_j] <= black_t { black_c += (black_j - black_i) as i64; black_i += 1; } else { black_j -= 1; } } black_c }; black_f(black_u) - black_f(black_l - 1) } }
```