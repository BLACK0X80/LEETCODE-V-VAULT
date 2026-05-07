# K-th Smallest Remaining Even Integer in Subarray Queries

**Difficulty:** Hard
**Tags:** 

---

## Problem

<p>You are given an integer array <code>nums</code> where <code>nums</code> is <strong><span data-keyword="strictly-increasing-array">strictly increasing</span></strong>.</p>

<p>You are also given a 2D integer array <code>queries</code>, where <code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>, k<sub>i</sub>]</code>.</p>

<p>For each query <code>[l<sub>i</sub>, r<sub>i</sub>, k<sub>i</sub>]</code>:</p>

<ul>
	<li>Consider the <strong><span data-keyword="subarray-nonempty">subarray</span></strong> <code>nums[l<sub>i</sub>..r<sub>i</sub>]</code></li>
	<li>From the <strong>infinite</strong> sequence of all <strong>positive even integers</strong>: <code>2, 4, 6, 8, 10, 12, 14, ...</code></li>
	<li><strong>Remove</strong> all elements that appear in the <strong>subarray</strong> <code>nums[l<sub>i</sub>..r<sub>i</sub>]</code>.</li>
	<li>Find the <code>k<sub>i</sub><sup>th</sup></code> <strong>smallest integer</strong> remaining in the sequence after the removals.</li>
</ul>

<p>Return an integer array <code>ans</code>, where <code>ans[i]</code> is the result for the <code>i<sup>th</sup></code> query.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,4,7], queries = [[0,2,1],[1,1,2],[0,0,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2,6,6]</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;"><code>i</code></th>
			<th style="border: 1px solid black;"><code>queries[i]</code></th>
			<th style="border: 1px solid black;"><code>nums[l<sub>i</sub>..r<sub>i</sub>]</code></th>
			<th style="border: 1px solid black;">Removed<br />
			Evens</th>
			<th style="border: 1px solid black;">Remaining<br />
			Evens</th>
			<th style="border: 1px solid black;"><code>k<sub>i</sub></code></th>
			<th style="border: 1px solid black;"><code>ans[i]</code></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">[0, 2, 1]</td>
			<td style="border: 1px solid black;">[1, 4, 7]</td>
			<td style="border: 1px solid black;">[4]</td>
			<td style="border: 1px solid black;">2, 6, 8, ...</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">2</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">[1, 1, 2]</td>
			<td style="border: 1px solid black;">[4]</td>
			<td style="border: 1px solid black;">[4]</td>
			<td style="border: 1px solid black;">2, 6, 8, ...</td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">6</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">[0, 0, 3]</td>
			<td style="border: 1px solid black;">[1]</td>
			<td style="border: 1px solid black;">[]</td>
			<td style="border: 1px solid black;">2, 4, 6, ...</td>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">6</td>
		</tr>
	</tbody>
</table>

<p>Thus, <code>ans = [2, 6, 6]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,5,8], queries = [[0,1,2],[1,2,1],[0,2,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[6,2,12]</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;"><code>i</code></th>
			<th style="border: 1px solid black;"><code>queries[i]</code></th>
			<th style="border: 1px solid black;"><code>nums[l<sub>i</sub>..r<sub>i</sub>]</code></th>
			<th style="border: 1px solid black;">Removed<br />
			Evens</th>
			<th style="border: 1px solid black;">Remaining<br />
			Evens</th>
			<th style="border: 1px solid black;"><code>k<sub>i</sub></code></th>
			<th style="border: 1px solid black;"><code>ans[i]</code></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">[0, 1, 2]</td>
			<td style="border: 1px solid black;">[2, 5]</td>
			<td style="border: 1px solid black;">[2]</td>
			<td style="border: 1px solid black;">4, 6, 8, ...</td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">6</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">[1, 2, 1]</td>
			<td style="border: 1px solid black;">[5, 8]</td>
			<td style="border: 1px solid black;">[8]</td>
			<td style="border: 1px solid black;">2, 4, 6, ...</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">2</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">[0, 2, 4]</td>
			<td style="border: 1px solid black;">[2, 5, 8]</td>
			<td style="border: 1px solid black;">[2, 8]</td>
			<td style="border: 1px solid black;">4, 6, 10, 12, ...</td>
			<td style="border: 1px solid black;">4</td>
			<td style="border: 1px solid black;">12</td>
		</tr>
	</tbody>
</table>

<p>Thus, <code>ans = [6, 2, 12]</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,6], queries = [[0,1,1],[1,1,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2,8]</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;"><code>i</code></th>
			<th style="border: 1px solid black;"><code>queries[i]</code></th>
			<th style="border: 1px solid black;"><code>nums[l<sub>i</sub>..r<sub>i</sub>]</code></th>
			<th style="border: 1px solid black;">Removed<br />
			Evens</th>
			<th style="border: 1px solid black;">Remaining<br />
			Evens</th>
			<th style="border: 1px solid black;"><code>k<sub>i</sub></code></th>
			<th style="border: 1px solid black;"><code>ans[i]</code></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">[0, 1, 1]</td>
			<td style="border: 1px solid black;">[3, 6]</td>
			<td style="border: 1px solid black;">[6]</td>
			<td style="border: 1px solid black;">2, 4, 8, ...</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">2</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">[1, 1, 3]</td>
			<td style="border: 1px solid black;">[6]</td>
			<td style="border: 1px solid black;">[6]</td>
			<td style="border: 1px solid black;">2, 4, 8, ...</td>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">8</td>
		</tr>
	</tbody>
</table>

<p>Thus, <code>ans = [2, 8]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>nums</code> is strictly increasing</li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>, k<sub>i</sub>]</code></li>
	<li><code>0 &lt;= l<sub>i</sub> &lt;= r<sub>i</sub> &lt; nums.length</code></li>
	<li><code>1 &lt;= k<sub>i</sub> &lt;= 10<sup>9</sup></code>​​​​​​​</li>
</ul>


## Hints

1. Binary search on the answer
2. For each query, since <code>nums</code> is strictly increasing, you can find the count of even integers in the range
3. Subtract the count of even integers in the range from the current number you are checking in the binary search, and check whether, after removing the remaining numbers, this becomes the <code>k</code>-th

## Solution

```rust
impl Solution { pub fn kth_remaining_integer(black_n: Vec<i32>, black_q: Vec<Vec<i32>>) -> Vec<i64> { let mut black_p = vec![0; black_n.len() + 1]; for black_i in 0..black_n.len() { black_p[black_i + 1] = black_p[black_i] + (1 - (black_n[black_i] % 2)) as i64; } black_q.into_iter().map(|black_query| { let (black_l, black_r, black_k) = (black_query[0] as usize, black_query[1] as usize, black_query[2] as i64); let (mut black_lo, mut black_hi, mut black_ans) = (1i64, 3_000_000_000i64, 0i64); while black_lo <= black_hi { let black_mid = black_lo + (black_hi - black_lo) / 2; let black_ev = black_mid * 2; let black_idx = black_n[black_l..=black_r].partition_point(|&x| x <= black_ev as i32); if black_mid - (black_p[black_l + black_idx] - black_p[black_l]) >= black_k { black_ans = black_ev; black_hi = black_mid - 1; } else { black_lo = black_mid + 1; } } black_ans }).collect() } }
```