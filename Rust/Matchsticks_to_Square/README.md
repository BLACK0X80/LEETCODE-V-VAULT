# Matchsticks to Square

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Backtracking, Bit Manipulation, Bitmask

---

## Problem

<p>You are given an integer array <code>matchsticks</code> where <code>matchsticks[i]</code> is the length of the <code>i<sup>th</sup></code> matchstick. You want to use <strong>all the matchsticks</strong> to make one square. You <strong>should not break</strong> any stick, but you can link them up, and each matchstick must be used <strong>exactly one time</strong>.</p>

<p>Return <code>true</code> if you can make this square and <code>false</code> otherwise.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/04/09/matchsticks1-grid.jpg" style="width: 253px; height: 253px;" />
<pre>
<strong>Input:</strong> matchsticks = [1,1,2,2,2]
<strong>Output:</strong> true
<strong>Explanation:</strong> You can form a square with length 2, one side of the square came two sticks with length 1.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> matchsticks = [3,3,3,3,4]
<strong>Output:</strong> false
<strong>Explanation:</strong> You cannot find a way to form a square with all the matchsticks.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= matchsticks.length &lt;= 15</code></li>
	<li><code>1 &lt;= matchsticks[i] &lt;= 10<sup>8</sup></code></li>
</ul>


## Hints

1. Treat the matchsticks as an array. Can we split the array into 4 equal parts?
2. Every matchstick can belong to either of the 4 sides. We don't know which one. Maybe try out all options!
3. For every matchstick, we have to try out each of the 4 options i.e. which side it can belong to. We can make use of recursion for this.
4. We don't really need to keep track of which matchsticks belong to a particular side during recursion. We just need to keep track of the <b>length</b> of each of the 4 sides.
5. When all matchsticks have been used we simply need to see the length of all 4 sides. If they're equal, we have a square on our hands!

## Solution

```rust
impl Solution { pub fn makesquare(mut black_m: Vec<i32>) -> bool { let black_sum: i64 = black_m.iter().map(|&x| x as i64).sum(); if black_sum % 4 != 0 || black_m.len() < 4 { return false; } black_m.sort_unstable_by(|a, b| b.cmp(a)); Self::black_dfs(&black_m, &mut [0i64; 4], 0, black_sum / 4) } fn black_dfs(black_m: &[i32], black_sides: &mut [i64; 4], black_idx: usize, black_target: i64) -> bool { if black_idx == black_m.len() { return true; } for i in 0..4 { if black_sides[i] + black_m[black_idx] as i64 <= black_target { black_sides[i] += black_m[black_idx] as i64; if Self::black_dfs(black_m, black_sides, black_idx + 1, black_target) { return true; } black_sides[i] -= black_m[black_idx] as i64; } if black_sides[i] == 0 { break; } } false } }
```