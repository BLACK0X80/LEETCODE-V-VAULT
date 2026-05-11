# Best Time to Buy and Sell Stock using Strategy

**Difficulty:** Medium
**Tags:** Array, Sliding Window, Prefix Sum

---

## Problem

<p>You are given two integer arrays <code>prices</code> and <code>strategy</code>, where:</p>

<ul>
	<li><code>prices[i]</code> is the price of a given stock on the <code>i<sup>th</sup></code> day.</li>
	<li><code>strategy[i]</code> represents a trading action on the <code>i<sup>th</sup></code> day, where:
	<ul>
		<li><code>-1</code> indicates buying one unit of the stock.</li>
		<li><code>0</code> indicates holding the stock.</li>
		<li><code>1</code> indicates selling one unit of the stock.</li>
	</ul>
	</li>
</ul>

<p>You are also given an <strong>even</strong> integer <code>k</code>, and may perform <strong>at most one</strong> modification to <code>strategy</code>. A modification consists of:</p>

<ul>
	<li>Selecting exactly <code>k</code> <strong>consecutive</strong> elements in <code>strategy</code>.</li>
	<li>Set the <strong>first</strong> <code>k / 2</code> elements to <code>0</code> (hold).</li>
	<li>Set the <strong>last</strong> <code>k / 2</code> elements to <code>1</code> (sell).</li>
</ul>

<p>The <strong>profit</strong> is defined as the <strong>sum</strong> of <code>strategy[i] * prices[i]</code> across all days.</p>

<p>Return the <strong>maximum</strong> possible profit you can achieve.</p>

<p><strong>Note:</strong> There are no constraints on budget or stock ownership, so all buy and sell operations are feasible regardless of past actions.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">prices = [4,2,8], strategy = [-1,0,1], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">10</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Modification</th>
			<th style="border: 1px solid black;">Strategy</th>
			<th style="border: 1px solid black;">Profit Calculation</th>
			<th style="border: 1px solid black;">Profit</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">Original</td>
			<td style="border: 1px solid black;">[-1, 0, 1]</td>
			<td style="border: 1px solid black;">(-1 &times; 4) + (0 &times; 2) + (1 &times; 8) = -4 + 0 + 8</td>
			<td style="border: 1px solid black;">4</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">Modify [0, 1]</td>
			<td style="border: 1px solid black;">[0, 1, 1]</td>
			<td style="border: 1px solid black;">(0 &times; 4) + (1 &times; 2) + (1 &times; 8) = 0 + 2 + 8</td>
			<td style="border: 1px solid black;">10</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">Modify [1, 2]</td>
			<td style="border: 1px solid black;">[-1, 0, 1]</td>
			<td style="border: 1px solid black;">(-1 &times; 4) + (0 &times; 2) + (1 &times; 8) = -4 + 0 + 8</td>
			<td style="border: 1px solid black;">4</td>
		</tr>
	</tbody>
</table>

<p>Thus, the maximum possible profit is 10, which is achieved by modifying the subarray <code>[0, 1]</code>​​​​​​​.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">prices = [5,4,3], strategy = [1,1,0], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">9</span></p>

<p><strong>Explanation:</strong></p>

<div class="example-block">
<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Modification</th>
			<th style="border: 1px solid black;">Strategy</th>
			<th style="border: 1px solid black;">Profit Calculation</th>
			<th style="border: 1px solid black;">Profit</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">Original</td>
			<td style="border: 1px solid black;">[1, 1, 0]</td>
			<td style="border: 1px solid black;">(1 &times; 5) + (1 &times; 4) + (0 &times; 3) = 5 + 4 + 0</td>
			<td style="border: 1px solid black;">9</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">Modify [0, 1]</td>
			<td style="border: 1px solid black;">[0, 1, 0]</td>
			<td style="border: 1px solid black;">(0 &times; 5) + (1 &times; 4) + (0 &times; 3) = 0 + 4 + 0</td>
			<td style="border: 1px solid black;">4</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">Modify [1, 2]</td>
			<td style="border: 1px solid black;">[1, 0, 1]</td>
			<td style="border: 1px solid black;">(1 &times; 5) + (0 &times; 4) + (1 &times; 3) = 5 + 0 + 3</td>
			<td style="border: 1px solid black;">8</td>
		</tr>
	</tbody>
</table>

<p>Thus, the maximum possible profit is 9, which is achieved without any modification.</p>
</div>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= prices.length == strategy.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= prices[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>-1 &lt;= strategy[i] &lt;= 1</code></li>
	<li><code>2 &lt;= k &lt;= prices.length</code></li>
	<li><code>k</code> is even</li>
</ul>


## Hints

1. Use prefix sums to precompute the base profit and to get fast range queries (sums of <code>prices</code> and counts of each <code>strategy</code> value over any interval).
2. Try every segment of length <code>k</code>: compute the profit delta caused by replacing that segment (using the prefix queries) and take the maximum of <code>base + delta</code>.

## Solution

```rust
impl Solution { pub fn max_profit(black_p: Vec<i32>, black_s: Vec<i32>, black_k: i32) -> i64 { let black_n = black_p.len(); let black_k = black_k as usize; let mut black_base: i64 = 0; let mut black_diff = vec![0i64; black_n]; for i in 0..black_n { black_base += (black_p[i] as i64) * (black_s[i] as i64); let black_target_sell = (black_p[i] as i64) * (1 - black_s[i] as i64); let black_target_hold = (black_p[i] as i64) * (0 - black_s[i] as i64); black_diff[i] = black_target_sell; } let mut black_pre_hold = vec![0i64; black_n + 1]; let mut black_pre_sell = vec![0i64; black_n + 1]; for i in 0..black_n { black_pre_hold[i + 1] = black_pre_hold[i] + (black_p[i] as i64) * (0 - black_s[i] as i64); black_pre_sell[i + 1] = black_pre_sell[i] + (black_p[i] as i64) * (1 - black_s[i] as i64); } let mut black_max_gain = 0i64; for i in 0..=black_n - black_k { let black_mid = i + black_k / 2; let black_gain = (black_pre_hold[black_mid] - black_pre_hold[i]) + (black_pre_sell[i + black_k] - black_pre_sell[black_mid]); black_max_gain = black_max_gain.max(black_gain); } black_base + black_max_gain } }
```