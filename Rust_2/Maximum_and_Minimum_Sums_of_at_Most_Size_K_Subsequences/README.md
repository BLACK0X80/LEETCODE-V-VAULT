# Maximum and Minimum Sums of at Most Size K Subsequences

**Difficulty:** Medium
**Tags:** Array, Math, Dynamic Programming, Sorting, Combinatorics

---

## Problem

<p>You are given an integer array <code>nums</code> and a positive integer <code>k</code>. Return the sum of the <strong>maximum</strong> and <strong>minimum</strong> elements of all <strong><span data-keyword="subsequence-sequence-nonempty">subsequences</span></strong> of <code>nums</code> with <strong>at most</strong> <code>k</code> elements.</p>

<p>Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3], k = 2</span></p>

<p><strong>Output:</strong> 24</p>

<p><strong>Explanation:</strong></p>

<p>The subsequences of <code>nums</code> with at most 2 elements are:</p>

<table style="border: 1px solid black;">
	<tbody>
		<tr>
			<th style="border: 1px solid black;"><b>Subsequence </b></th>
			<th style="border: 1px solid black;">Minimum</th>
			<th style="border: 1px solid black;">Maximum</th>
			<th style="border: 1px solid black;">Sum</th>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[1]</code></td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">2</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[2]</code></td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">4</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[3]</code></td>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">6</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[1, 2]</code></td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">3</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[1, 3]</code></td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">4</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[2, 3]</code></td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">5</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><strong>Final Total</strong></td>
			<td style="border: 1px solid black;">&nbsp;</td>
			<td style="border: 1px solid black;">&nbsp;</td>
			<td style="border: 1px solid black;">24</td>
		</tr>
	</tbody>
</table>

<p>The output would be 24.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [5,0,6], k = 1</span></p>

<p><strong>Output:</strong> 2<span class="example-io">2</span></p>

<p><strong>Explanation: </strong></p>

<p>For subsequences with exactly 1 element, the minimum and maximum values are the element itself. Therefore, the total is <code>5 + 5 + 0 + 0 + 6 + 6 = 22</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1,1], k = 2</span></p>

<p><strong>Output:</strong> 12</p>

<p><strong>Explanation:</strong></p>

<p>The subsequences <code>[1, 1]</code> and <code>[1]</code> each appear 3 times. For all of them, the minimum and maximum are both 1. Thus, the total is 12.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code><font face="monospace">1 &lt;= k &lt;= min(70, nums.length)</font></code></li>
</ul>


## Hints

1. Sort the array.

## Solution

```rust
impl Solution { pub fn min_max_sums(mut nums: Vec<i32>, k: i32) -> i32 { nums.sort(); let (n, black_mod) = (nums.len(), 1_000_000_007i64); let mut black_c = vec![vec![0i64; k as usize + 1]; n + 1]; for i in 0..=n { black_c[i][0] = 1; for j in 1..=i.min(k as usize) { black_c[i][j] = (black_c[i-1][j-1] + black_c[i-1][j]) % black_mod; } } let (mut black_ans, mut black_comb) = (0i64, vec![0i64; n]); for i in 0..n { let mut s = 0; for j in 0..k as usize { s = (s + black_c[i][j]) % black_mod; } black_comb[i] = s; } for i in 0..n { black_ans = (black_ans + nums[i] as i64 * (black_comb[i] + black_comb[n-1-i])) % black_mod; } black_ans as i32 } }
```