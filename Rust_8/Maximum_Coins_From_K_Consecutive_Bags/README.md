# Maximum Coins From K Consecutive Bags

**Difficulty:** Medium
**Tags:** Array, Binary Search, Greedy, Sliding Window, Sorting, Prefix Sum

---

## Problem

<p>There are an infinite amount of bags on a number line, one bag for each coordinate. Some of these bags contain coins.</p>

<p>You are given a 2D array <code>coins</code>, where <code>coins[i] = [l<sub>i</sub>, r<sub>i</sub>, c<sub>i</sub>]</code> denotes that every bag from <code>l<sub>i</sub></code> to <code>r<sub>i</sub></code> contains <code>c<sub>i</sub></code> coins.</p>

<p>The segments that <code>coins</code> contain are non-overlapping.</p>

<p>You are also given an integer <code>k</code>.</p>

<p>Return the <strong>maximum</strong> amount of coins you can obtain by collecting <code>k</code> consecutive bags.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">coins = [[8,10,1],[1,3,2],[5,6,4]], k = 4</span></p>

<p><strong>Output:</strong> <span class="example-io">10</span></p>

<p><strong>Explanation:</strong></p>

<p>Selecting bags at positions <code>[3, 4, 5, 6]</code> gives the maximum number of coins:&nbsp;<code>2 + 0 + 4 + 4 = 10</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">coins = [[1,10,3]], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>Selecting bags at positions <code>[1, 2]</code> gives the maximum number of coins:&nbsp;<code>3 + 3 = 6</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= coins.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>9</sup></code></li>
	<li><code>coins[i] == [l<sub>i</sub>, r<sub>i</sub>, c<sub>i</sub>]</code></li>
	<li><code>1 &lt;= l<sub>i</sub> &lt;= r<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= c<sub>i</sub> &lt;= 1000</code></li>
	<li>The given segments are non-overlapping.</li>
</ul>


## Hints

1. An optimal starting position for <code>k</code> consecutive bags will be either <code>l<sub>i</sub></code> or <code>r<sub>i</sub> - k + 1</code>.

## Solution

```rust
impl Solution { pub fn maximum_coins(mut black_c: Vec<Vec<i32>>, k: i32) -> i64 { black_c.sort_unstable(); let (n, mut black_ans, mut black_curr, mut black_j) = (black_c.len(), 0i64, 0i64, 0); for i in 0..n { while black_j < n && black_c[black_j][1] <= black_c[i][0] + k - 1 { black_curr += (black_c[black_j][1] - black_c[black_j][0] + 1) as i64 * black_c[black_j][2] as i64; black_j += 1; } if black_j < n { black_ans = black_ans.max(black_curr + (black_c[i][0] + k - 1 - black_c[black_j][0] + 1).max(0) as i64 * black_c[black_j][2] as i64); } black_curr -= (black_c[i][1] - black_c[i][0] + 1) as i64 * black_c[i][2] as i64; } (black_curr, black_j) = (0, 0); for i in 0..n { black_curr += (black_c[i][1] - black_c[i][0] + 1) as i64 * black_c[i][2] as i64; while black_c[black_j][1] < black_c[i][1] - k + 1 { black_curr -= (black_c[black_j][1] - black_c[black_j][0] + 1) as i64 * black_c[black_j][2] as i64; black_j += 1; } black_ans = black_ans.max(black_curr - (black_c[i][1] - k - black_c[black_j][0] + 1).max(0) as i64 * black_c[black_j][2] as i64); } black_ans } }
```