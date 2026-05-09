# Last Stone Weight II

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given an array of integers <code>stones</code> where <code>stones[i]</code> is the weight of the <code>i<sup>th</sup></code> stone.</p>

<p>We are playing a game with the stones. On each turn, we choose any two stones and smash them together. Suppose the stones have weights <code>x</code> and <code>y</code> with <code>x &lt;= y</code>. The result of this smash is:</p>

<ul>
	<li>If <code>x == y</code>, both stones are destroyed, and</li>
	<li>If <code>x != y</code>, the stone of weight <code>x</code> is destroyed, and the stone of weight <code>y</code> has new weight <code>y - x</code>.</li>
</ul>

<p>At the end of the game, there is <strong>at most one</strong> stone left.</p>

<p>Return <em>the smallest possible weight of the left stone</em>. If there are no stones left, return <code>0</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> stones = [2,7,4,1,8,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
We can combine 2 and 4 to get 2, so the array converts to [2,7,1,8,1] then,
we can combine 7 and 8 to get 1, so the array converts to [2,1,1,1] then,
we can combine 2 and 1 to get 1, so the array converts to [1,1,1] then,
we can combine 1 and 1 to get 0, so the array converts to [1], then that&#39;s the optimal value.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> stones = [31,26,33,21,40]
<strong>Output:</strong> 5
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= stones.length &lt;= 30</code></li>
	<li><code>1 &lt;= stones[i] &lt;= 100</code></li>
</ul>


## Hints

1. Think of the final answer as a sum of weights with + or - sign symbols infront of each weight.  Actually, all sums with 1 of each sign symbol are possible.
2. Use dynamic programming: for every possible sum with N stones, those sums +x or -x is possible with N+1 stones, where x is the value of the newest stone.  (This overcounts sums that are all positive or all negative, but those don't matter.)

## Solution

```rust
impl Solution { pub fn last_stone_weight_ii(black_stones: Vec<i32>) -> i32 { let black_sum: i32 = black_stones.iter().sum(); let mut black_dp = vec![false; (black_sum / 2 + 1) as usize]; black_dp[0] = true; for &black_s in &black_stones { for black_i in (black_s as usize..black_dp.len()).rev() { if black_dp[black_i - black_s as usize] { black_dp[black_i] = true; } } } (black_sum - 2 * black_dp.iter().enumerate().filter(|&(_, &v)| v).map(|(i, _)| i as i32).last().unwrap()) } }
```