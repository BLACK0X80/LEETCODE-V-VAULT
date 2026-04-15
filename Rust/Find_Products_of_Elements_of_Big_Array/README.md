# Find Products of Elements of Big Array

**Difficulty:** Hard
**Tags:** Array, Binary Search, Bit Manipulation

---

## Problem

<p>The <strong>powerful array</strong> of a non-negative integer <code>x</code> is defined as the shortest sorted array of powers of two that sum up to <code>x</code>. The table below illustrates examples of how the <strong>powerful array</strong> is determined. It can be proven that the powerful array of <code>x</code> is unique.</p>

<table border="1">
	<tbody>
		<tr>
			<th>num</th>
			<th>Binary Representation</th>
			<th>powerful array</th>
		</tr>
		<tr>
			<td>1</td>
			<td>0000<u>1</u></td>
			<td>[1]</td>
		</tr>
		<tr>
			<td>8</td>
			<td>0<u>1</u>000</td>
			<td>[8]</td>
		</tr>
		<tr>
			<td>10</td>
			<td>0<u>1</u>0<u>1</u>0</td>
			<td>[2, 8]</td>
		</tr>
		<tr>
			<td>13</td>
			<td>0<u>11</u>0<u>1</u></td>
			<td>[1, 4, 8]</td>
		</tr>
		<tr>
			<td>23</td>
			<td><u>1</u>0<u>111</u></td>
			<td>[1, 2, 4, 16]</td>
		</tr>
	</tbody>
</table>

<p>The array <code>big_nums</code> is created by concatenating the <strong>powerful arrays</strong> for every positive integer <code>i</code> in ascending order: 1, 2, 3, and so on. Thus, <code>big_nums</code> begins as <code>[<u>1</u>, <u>2</u>, <u>1, 2</u>, <u>4</u>, <u>1, 4</u>, <u>2, 4</u>, <u>1, 2, 4</u>, <u>8</u>, ...]</code>.</p>

<p>You are given a 2D integer matrix <code>queries</code>, where for <code>queries[i] = [from<sub>i</sub>, to<sub>i</sub>, mod<sub>i</sub>]</code> you should calculate <code>(big_nums[from<sub>i</sub>] * big_nums[from<sub>i</sub> + 1] * ... * big_nums[to<sub>i</sub>]) % mod<sub>i</sub></code><!-- notionvc: a71131cc-7b52-4786-9a4b-660d6d864f89 -->.</p>

<p>Return an integer array <code>answer</code> such that <code>answer[i]</code> is the answer to the <code>i<sup>th</sup></code> query.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">queries = [[1,3,7]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[4]</span></p>

<p><strong>Explanation:</strong></p>

<p>There is one query.</p>

<p><code>big_nums[1..3] = [2,1,2]</code>. The product of them is 4. The result is <code>4 % 7 = 4.</code></p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">queries = [[2,5,3],[7,7,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2,2]</span></p>

<p><strong>Explanation:</strong></p>

<p>There are two queries.</p>

<p>First query: <code>big_nums[2..5] = [1,2,4,1]</code>. The product of them is 8. The result is <code>8 % 3 = 2</code>.</p>

<p>Second query: <code>big_nums[7] = 2</code>. The result is <code>2 % 4 = 2</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= queries.length &lt;= 500</code></li>
	<li><code>queries[i].length == 3</code></li>
	<li><code>0 &lt;= queries[i][0] &lt;= queries[i][1] &lt;= 10<sup>15</sup></code></li>
	<li><code>1 &lt;= queries[i][2] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Find a way to calculate <code>f(n, i)</code> which is the total number of numbers in <code>[1, n]</code> when the <code>i<sup>th</sup></code> bit is set in <code>O(log(n))</code> time.
2. Use binary search to find the last number for each query (and there might be one “incomplete” number for the query).
3. Use a similar way to find the product (we only need to save the sum of exponents of power of <code>2</code>).

## Solution

```rust
impl Solution {
    pub fn find_products_of_elements(black_queries: Vec<Vec<i64>>) -> Vec<i32> {
        let black_count_bits = |mut black_n: i64| { let (mut black_c, mut black_temp) = (0, black_n); for black_i in 0..50 { let black_full = (black_n + 1) / (1 << (black_i + 1)); black_c += black_full * (1 << black_i) + 0.max((black_n + 1) % (1 << (black_i + 1)) - (1 << black_i)); } black_c };
        let black_sum_powers = |mut black_n: i64| { let mut black_s = 0; for black_i in 0..50 { let black_full = (black_n + 1) / (1 << (black_i + 1)); black_s += (black_full * (1 << black_i) + 0.max((black_n + 1) % (1 << (black_i + 1)) - (1 << black_i))) * black_i as i64; } black_s };
        let black_solve = |black_idx: i64| {
            let (mut black_l, mut black_r, mut black_num) = (0, 1e15 as i64, 0);
            while black_l <= black_r { let black_m = (black_l + black_r) / 2; if black_count_bits(black_m) <= black_idx { black_num = black_m; black_l = black_m + 1; } else { black_r = black_m - 1; } }
            let (mut black_res, mut black_rem) = (black_sum_powers(black_num), black_idx - black_count_bits(black_num));
            for black_i in 0..50 { if black_rem > 0 && ((black_num + 1) >> black_i) & 1 == 1 { black_res += black_i as i64; black_rem -= 1; } }
            black_res
        };
        black_queries.iter().map(|black_q| {
            let black_p = black_solve(black_q[1] + 1) - black_solve(black_q[0]);
            let (mut black_base, mut black_exp, mut black_res, black_mod) = (2i64, black_p, 1i64, black_q[2]);
            while black_exp > 0 { if black_exp % 2 == 1 { black_res = (black_res * black_base) % black_mod; } black_base = (black_base * black_base) % black_mod; black_exp /= 2; }
            (black_res % black_mod) as i32
        }).collect()
    }
}
```