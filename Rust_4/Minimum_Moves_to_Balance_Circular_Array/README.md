# Minimum Moves to Balance Circular Array

**Difficulty:** Medium
**Tags:** Array, Greedy, Sorting

---

## Problem

<p>You are given a <strong>circular</strong> array <code>balance</code> of length <code>n</code>, where <code>balance[i]</code> is the net balance of person <code>i</code>.</p>

<p>In one move, a person can transfer <strong>exactly</strong> 1 unit of balance to either their left or right neighbor.</p>

<p>Return the <strong>minimum</strong> number of moves required so that every person has a <strong>non-negative</strong> balance. If it is impossible, return <code>-1</code>.</p>

<p><strong>Note</strong>: You are guaranteed that <strong>at most</strong> 1 index has a <strong>negative</strong> balance initially.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">balance = [5,1,-4]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>One optimal sequence of moves is:</p>

<ul>
	<li>Move 1 unit from <code>i = 1</code> to <code>i = 2</code>, resulting in <code>balance = [5, 0, -3]</code></li>
	<li>Move 1 unit from <code>i = 0</code> to <code>i = 2</code>, resulting in <code>balance = [4, 0, -2]</code></li>
	<li>Move 1 unit from <code>i = 0</code> to <code>i = 2</code>, resulting in <code>balance = [3, 0, -1]</code></li>
	<li>Move 1 unit from <code>i = 0</code> to <code>i = 2</code>, resulting in <code>balance = [2, 0, 0]</code></li>
</ul>

<p>Thus, the minimum number of moves required is 4.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">balance = [1,2,-5,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>One optimal sequence of moves is:</p>

<ul>
	<li>Move 1 unit from <code>i = 1</code> to <code>i = 2</code>, resulting in <code>balance = [1, 1, -4, 2]</code></li>
	<li>Move 1 unit from <code>i = 1</code> to <code>i = 2</code>, resulting in <code>balance = [1, 0, -3, 2]</code></li>
	<li>Move 1 unit from <code>i = 3</code> to <code>i = 2</code>, resulting in <code>balance = [1, 0, -2, 1]</code></li>
	<li>Move 1 unit from <code>i = 3</code> to <code>i = 2</code>, resulting in <code>balance = [1, 0, -1, 0]</code></li>
	<li>Move 1 unit from <code>i = 0</code> to <code>i = 1</code>, resulting in <code>balance = [0, 1, -1, 0]</code></li>
	<li>Move 1 unit from <code>i = 1</code> to <code>i = 2</code>, resulting in <code>balance = [0, 0, 0, 0]</code></li>
</ul>

<p>Thus, the minimum number of moves required is 6.​​​</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">balance = [-3,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p><strong>​​​​​​​</strong>It is impossible to make all balances non-negative for <code>balance = [-3, 2]</code>, so the answer is -1.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == balance.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>9</sup> &lt;= balance[i] &lt;= 10<sup>9</sup></code></li>
	<li>There is at most one negative value in <code>balance</code> initially.</li>
</ul>


## Hints

1. If there is no negative value, then the answer is 0. If the total sum is less than 0, then the answer is -1.
2. Sort the positive values in <code>nums</code> by their distance from the index with the negative value.
3. Greedily use as many values as needed from the sorted <code>nums</code> to offset the current negative value.

## Solution

```rust
impl Solution { pub fn min_moves(balance: Vec<i32>) -> i64 { let black_sum: i64 = balance.iter().map(|&black_x| black_x as i64).sum(); if black_sum < 0 { return -1; } let mut black_neg = -1; for black_i in 0..balance.len() { if balance[black_i] < 0 { black_neg = black_i as i32; break; } } if black_neg == -1 { return 0; } let mut black_req = -balance[black_neg as usize] as i64; let mut black_dists = Vec::new(); let black_n = balance.len() as i32; for black_i in 0..black_n { if black_i != black_neg && balance[black_i as usize] > 0 { let black_d1 = (black_i - black_neg).abs(); let black_d2 = black_n - black_d1; black_dists.push((black_d1.min(black_d2) as i64, balance[black_i as usize] as i64)); } } black_dists.sort_unstable_by_key(|black_x| black_x.0); let mut black_ans = 0i64; for (black_d, black_v) in black_dists { let black_take = black_v.min(black_req); black_ans += black_take * black_d; black_req -= black_take; if black_req == 0 { break; } } black_ans } }
```