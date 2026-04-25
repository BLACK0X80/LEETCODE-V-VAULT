# Stone Game II

**Difficulty:** Medium
**Tags:** Array, Math, Dynamic Programming, Prefix Sum, Game Theory

---

## Problem

<p>Alice and Bob continue their games with piles of stones. There are a number of piles <strong>arranged in a row</strong>, and each pile has a positive integer number of stones <code>piles[i]</code>. The objective of the game is to end with the most stones.</p>

<p>Alice and Bob take turns, with Alice starting first.</p>

<p>On each player&#39;s turn, that player can take <strong>all the stones</strong> in the <strong>first</strong> <code>X</code> remaining piles, where <code>1 &lt;= X &lt;= 2M</code>. Then, we set <code>M = max(M, X)</code>. Initially, M = 1.</p>

<p>The game continues until all the stones have been taken.</p>

<p>Assuming Alice and Bob play optimally, return the maximum number of stones Alice can get.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">piles = [2,7,9,4,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">10</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>If Alice takes one pile at the beginning, Bob takes two piles, then Alice takes 2 piles again. Alice can get <code>2 + 4 + 4 = 10</code> stones in total.</li>
	<li>If Alice takes two piles at the beginning, then Bob can take all three piles left. In this case, Alice get <code>2 + 7 = 9</code> stones in total.</li>
</ul>

<p>So we return 10 since it&#39;s larger.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">piles = [1,2,3,4,5,100]</span></p>

<p><strong>Output:</strong> <span class="example-io">104</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= piles.length &lt;= 100</code></li>
	<li><code>1 &lt;= piles[i]&nbsp;&lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Use dynamic programming: the states are (i, m) for the answer of piles[i:] and that given m.

## Solution

```rust
impl Solution { pub fn stone_game_ii(black_p: Vec<i32>) -> i32 { let black_n = black_p.len(); let mut black_s = vec![0; black_n + 1]; for black_i in (0..black_n).rev() { black_s[black_i] = black_s[black_i+1] + black_p[black_i]; } let mut black_dp = vec![vec![0; black_n + 1]; black_n + 1]; for black_i in (0..black_n).rev() { for black_m in 1..=black_n { if black_i + 2 * black_m >= black_n { black_dp[black_i][black_m] = black_s[black_i]; } else { for black_x in 1..=2 * black_m { black_dp[black_i][black_m] = black_dp[black_i][black_m].max(black_s[black_i] - black_dp[black_i+black_x][black_m.max(black_x)]); } } } } black_dp[0][1] } }
```