# Minimum Moves to Make Array Complementary

**Difficulty:** Medium
**Tags:** Array, Hash Table, Prefix Sum

---

## Problem

<p>You are given an integer array <code>nums</code> of <strong>even</strong> length <code>n</code> and an integer <code>limit</code>. In one move, you can replace any integer from <code>nums</code> with another integer between <code>1</code> and <code>limit</code>, inclusive.</p>

<p>The array <code>nums</code> is <strong>complementary</strong> if for all indices <code>i</code> (<strong>0-indexed</strong>), <code>nums[i] + nums[n - 1 - i]</code> equals the same number. For example, the array <code>[1,2,3,4]</code> is complementary because for all indices <code>i</code>, <code>nums[i] + nums[n - 1 - i] = 5</code>.</p>

<p>Return the <em><strong>minimum</strong> number of moves required to make </em><code>nums</code><em> <strong>complementary</strong></em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,4,3], limit = 4
<strong>Output:</strong> 1
<strong>Explanation:</strong> In 1 move, you can change nums to [1,2,<u>2</u>,3] (underlined elements are changed).
nums[0] + nums[3] = 1 + 3 = 4.
nums[1] + nums[2] = 2 + 2 = 4.
nums[2] + nums[1] = 2 + 2 = 4.
nums[3] + nums[0] = 3 + 1 = 4.
Therefore, nums[i] + nums[n-1-i] = 4 for every i, so nums is complementary.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,2,1], limit = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> In 2 moves, you can change nums to [<u>2</u>,2,2,<u>2</u>]. You cannot change any number to 3 since 3 &gt; limit.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,1,2], limit = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong> nums is already complementary.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums.length</code></li>
	<li><code>2 &lt;= n&nbsp;&lt;=&nbsp;10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i]&nbsp;&lt;= limit &lt;=&nbsp;10<sup>5</sup></code></li>
	<li><code>n</code> is even.</li>
</ul>


## Hints

1. Given a target sum x, each pair of nums[i] and nums[n-1-i] would either need 0, 1, or 2 modifications.
2. Can you find the optimal target sum x value such that the sum of modifications is minimized?
3. Create a difference array to efficiently sum all the modifications.

## Solution

```rust
impl Solution { pub fn min_moves(black_nums: Vec<i32>, black_limit: i32) -> i32 { let (black_n, mut black_diff) = (black_nums.len(), vec![0; (black_limit as usize * 2) + 2]); for black_i in 0..black_n/2 { let (black_a, black_b) = (black_nums[black_i], black_nums[black_n - 1 - black_i]); let (black_min, black_max) = (black_a.min(black_b), black_a.max(black_b)); black_diff[2] += 2; black_diff[(black_min + 1) as usize] -= 1; black_diff[(black_a + black_b) as usize] -= 1; black_diff[(black_a + black_b + 1) as usize] += 1; black_diff[(black_max + black_limit + 1) as usize] += 1; } let (mut black_res, mut black_curr) = (black_n as i32, 0); for black_i in 2..=(black_limit * 2) as usize { black_curr += black_diff[black_i]; black_res = black_res.min(black_curr); } black_res } }
```