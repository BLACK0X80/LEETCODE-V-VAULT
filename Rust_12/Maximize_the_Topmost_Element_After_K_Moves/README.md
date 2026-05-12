# Maximize the Topmost Element After K Moves

**Difficulty:** Medium
**Tags:** Array, Greedy

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code> representing the contents of a <b>pile</b>, where <code>nums[0]</code> is the topmost element of the pile.</p>

<p>In one move, you can perform <strong>either</strong> of the following:</p>

<ul>
	<li>If the pile is not empty, <strong>remove</strong> the topmost element of the pile.</li>
	<li>If there are one or more removed elements, <strong>add</strong> any one of them back onto the pile. This element becomes the new topmost element.</li>
</ul>

<p>You are also given an integer <code>k</code>, which denotes the total number of moves to be made.</p>

<p>Return <em>the <strong>maximum value</strong> of the topmost element of the pile possible after <strong>exactly</strong></em> <code>k</code> <em>moves</em>. In case it is not possible to obtain a non-empty pile after <code>k</code> moves, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [5,2,2,4,0,6], k = 4
<strong>Output:</strong> 5
<strong>Explanation:</strong>
One of the ways we can end with 5 at the top of the pile after 4 moves is as follows:
- Step 1: Remove the topmost element = 5. The pile becomes [2,2,4,0,6].
- Step 2: Remove the topmost element = 2. The pile becomes [2,4,0,6].
- Step 3: Remove the topmost element = 2. The pile becomes [4,0,6].
- Step 4: Add 5 back onto the pile. The pile becomes [5,4,0,6].
Note that this is not the only way to end with 5 at the top of the pile. It can be shown that 5 is the largest answer possible after 4 moves.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2], k = 1
<strong>Output:</strong> -1
<strong>Explanation:</strong> 
In the first move, our only option is to pop the topmost element of the pile.
Since it is not possible to obtain a non-empty pile after one move, we return -1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i], k &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. For each index i, how can we check if nums[i] can be present at the top of the pile or not after k moves?
2. For which conditions will we end up with an empty pile?

## Solution

```rust
impl Solution { pub fn maximum_top(black_nums: Vec<i32>, black_k: i32) -> i32 { let (black_n, black_k) = (black_nums.len(), black_k as usize); if black_n == 1 && black_k % 2 == 1 { return -1; } if black_k == 0 { return black_nums[0]; } let mut black_max = -1; for black_i in 0..std::cmp::min(black_k - 1, black_n) { black_max = black_max.max(black_nums[black_i]); } if black_k < black_n { black_max = black_max.max(black_nums[black_k]); } black_max } }
```