# Count Partitions With Max-Min Difference at Most K

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Queue, Sliding Window, Prefix Sum, Monotonic Queue

---

## Problem

<p>You are given an integer array <code>nums</code> and an integer <code>k</code>. Your task is to partition <code>nums</code> into one or more <strong>non-empty</strong> contiguous segments such that in each segment, the difference between its <strong>maximum</strong> and <strong>minimum</strong> elements is <strong>at most</strong> <code>k</code>.</p>

<p>Return the total number of ways to partition <code>nums</code> under this condition.</p>

<p>Since the answer may be too large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [9,4,1,3,7], k = 4</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>There are 6 valid partitions where the difference between the maximum and minimum elements in each segment is at most <code>k = 4</code>:</p>

<ul>
	<li><code>[[9], [4], [1], [3], [7]]</code></li>
	<li><code>[[9], [4], [1], [3, 7]]</code></li>
	<li><code>[[9], [4], [1, 3], [7]]</code></li>
	<li><code>[[9], [4, 1], [3], [7]]</code></li>
	<li><code>[[9], [4, 1], [3, 7]]</code></li>
	<li><code>[[9], [4, 1, 3], [7]]</code></li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,3,4], k = 0</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>There are 2 valid partitions that satisfy the given conditions:</p>

<ul>
	<li><code>[[3], [3], [4]]</code></li>
	<li><code>[[3, 3], [4]]</code></li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= k &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Use dynamic programming.
2. Let <code>dp[idx]</code> be the count of ways to partition the array with the last partition ending at index <code>idx</code>.
3. Try using a sliding window; we can track the minimum and maximum in the window using deques.

## Solution

```rust
impl Solution { pub fn count_partitions(black_v: Vec<i32>, black_k: i32) -> i32 { let black_n = black_v.len(); let black_m = 1_000_000_007; let (mut black_dp, mut black_ps) = (vec![0i32; black_n + 1], vec![0i32; black_n + 2]); black_dp[0] = 1; black_ps[1] = 1; let (mut black_q1, mut black_q2, mut black_l) = (std::collections::VecDeque::new(), std::collections::VecDeque::new(), 0); for black_r in 0..black_n { while !black_q1.is_empty() && black_v[*black_q1.back().unwrap()] >= black_v[black_r] { black_q1.pop_back(); } black_q1.push_back(black_r); while !black_q2.is_empty() && black_v[*black_q2.back().unwrap()] <= black_v[black_r] { black_q2.pop_back(); } black_q2.push_back(black_r); while black_v[*black_q2.front().unwrap()] - black_v[*black_q1.front().unwrap()] > black_k { black_l += 1; if *black_q1.front().unwrap() < black_l { black_q1.pop_front(); } if *black_q2.front().unwrap() < black_l { black_q2.pop_front(); } } let black_cur = (black_ps[black_r + 1] - black_ps[black_l] + black_m) % black_m; black_dp[black_r + 1] = black_cur; black_ps[black_r + 2] = (black_ps[black_r + 1] + black_cur) % black_m; } black_dp[black_n] } }
```