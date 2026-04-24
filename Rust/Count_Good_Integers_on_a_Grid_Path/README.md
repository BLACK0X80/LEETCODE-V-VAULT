# Count Good Integers on a Grid Path

**Difficulty:** Hard
**Tags:** Dynamic Programming

---

## Problem

<p>You are given two integers <code>l</code> and <code>r</code>, and a string <code>directions</code> consisting of <strong>exactly</strong> three <code>&#39;D&#39;</code> characters and three <code>&#39;R&#39;</code> characters.</p>

<p>For each integer <code>x</code> in the range <code>[l, r]</code> (inclusive), perform the following steps:</p>

<ol>
	<li>If <code>x</code> has fewer than 16 digits, pad it on the left with <strong>leading zeros</strong> to obtain a 16-digit string.</li>
	<li>Place the 16 digits into a <code>4 &times; 4</code> grid in <strong>row-major</strong> order (the first 4 digits form the first row from left to right, the next 4 digits form the second row, and so on).</li>
	<li>Starting at the <strong>top-left</strong> cell (<code>row = 0</code>, <code>column = 0</code>), apply the 6 characters of <code>directions</code> in order:
	<ul>
		<li><code>&#39;D&#39;</code> increments the row by 1.</li>
		<li><code>&#39;R&#39;</code> increments the column by 1.</li>
	</ul>
	</li>
	<li>Record the sequence of digits visited along the path (including the starting cell), producing a sequence of length 7.</li>
</ol>

<p>The integer <code>x</code> is considered <strong>good</strong> if the recorded sequence is <strong>non-decreasing</strong>.</p>

<p>Return an integer representing the number of good integers in the range <code>[l, r]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = 8, r = 10, directions = &quot;DDDRRR&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The grid for <code>x = 8</code>:</p>

<table style="border: 1px solid black;">
	<tbody>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">8</td>
		</tr>
	</tbody>
</table>

<ul>
	<li>Path: <code>(0,0) &rarr; (1,0) &rarr; (2,0) &rarr; (3,0) &rarr; (3,1) &rarr; (3,2) &rarr; (3,3)</code></li>
	<li>The sequence of digits visited is <code>[0, 0, 0, 0, 0, 0, 8]</code>.</li>
	<li>As the sequence of digits visited is non-decreasing, 8 is a good integer.</li>
</ul>

<p>The grid for <code>x = 9</code>:</p>

<table style="border: 1px solid black;">
	<tbody>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">9</td>
		</tr>
	</tbody>
</table>

<ul>
	<li>The sequence of digits visited is <code>[0, 0, 0, 0, 0, 0, 9]</code>.</li>
	<li>As the sequence of digits visited is non-decreasing, 9 is a good integer.</li>
</ul>

<p>The grid for <code>x = 10</code>:</p>

<table style="border: 1px solid black;">
	<tbody>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
	</tbody>
</table>

<ul>
	<li>The sequence of digits visited is <code>[0, 0, 0, 0, 0, 1, 0]</code>.</li>
	<li>As the sequence of digits visited is not non-decreasing, 10 is not a good integer.</li>
	<li>Hence, only 8 and 9 are good, giving a total of 2 good integers in the range.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = 123456789, r = 123456790, directions = &quot;DDRRDR&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The grid for <code>x = 123456789</code>:</p>

<table style="border: 1px solid black;">
	<tbody>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">4</td>
			<td style="border: 1px solid black;">5</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">6</td>
			<td style="border: 1px solid black;">7</td>
			<td style="border: 1px solid black;">8</td>
			<td style="border: 1px solid black;">9</td>
		</tr>
	</tbody>
</table>

<ul>
	<li>Path: <code>(0,0) &rarr; (1,0) &rarr; (2,0) &rarr; (2,1) &rarr; (2,2) &rarr; (3,2) &rarr; (3,3)</code></li>
	<li>The sequence of digits visited is <code>[0, 0, 2, 3, 4, 8, 9]</code>.</li>
	<li>As the sequence of digits visited is non-decreasing, 123456789 is a good integer.</li>
</ul>

<p>The grid for <code>x = 123456790</code>:</p>

<table style="border: 1px solid black;">
	<tbody>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">4</td>
			<td style="border: 1px solid black;">5</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">6</td>
			<td style="border: 1px solid black;">7</td>
			<td style="border: 1px solid black;">9</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
	</tbody>
</table>

<ul>
	<li>The sequence of digits visited is <code>[0, 0, 2, 3, 4, 9, 0]</code>.</li>
	<li>As the sequence of digits visited is not non-decreasing, 123456790 is not a good integer.</li>
	<li>Hence, only 123456789 is good, giving a total of 1 good integer in the range.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = 1288561398769758, r = 1288561398769758, directions = &quot;RRRDDD&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>The grid for <code>x = 1288561398769758</code>:</p>

<table style="border: 1px solid black;">
	<tbody>
		<tr style="background:none;">
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">8</td>
			<td style="border: 1px solid black;">8</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">5</td>
			<td style="border: 1px solid black;">6</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">3</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">9</td>
			<td style="border: 1px solid black;">8</td>
			<td style="border: 1px solid black;">7</td>
			<td style="border: 1px solid black;">6</td>
		</tr>
		<tr style="background:none;">
			<td style="border: 1px solid black;">9</td>
			<td style="border: 1px solid black;">7</td>
			<td style="border: 1px solid black;">5</td>
			<td style="border: 1px solid black;">8</td>
		</tr>
	</tbody>
</table>

<ul>
	<li>Path: <code>(0,0) &rarr; (0,1) &rarr; (0,2) &rarr; (0,3) &rarr; (1,3) &rarr; (2,3) &rarr; (3,3)</code></li>
	<li>The sequence of digits visited is <code>[1, 2, 8, 8, 3, 6, 8]</code>.</li>
	<li>​​​​​​​As the sequence of digits visited is not non-decreasing, 1288561398769758 is not a good integer.</li>
	<li>No numbers are good, giving a total of 0 good integers in the range.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= l &lt;= r &lt;= 9 &times; 10<sup>15</sup></code></li>
	<li><code>directions.length == 6</code></li>
	<li><code>directions</code> consists of <strong>exactly</strong> three <code>&#39;D&#39;</code> characters and three <code>&#39;R&#39;</code> characters.</li>
</ul>


## Hints

1. Simulate the path to get the 7 increasing indices; since moves are only down/right, the visited positions are in left-to-right digit order, so you only need to check those 7 digits are non-decreasing.
2. Use digit dynamic programming with states <code>pos</code> (0..15), <code>tight</code> (0/1), <code>cnt</code> (how many path digits placed 0..7), and <code>prev</code> (previous path digit 0..9); when <code>pos</code> is a path index, require the chosen digit >= <code>prev</code> and advance <code>cnt</code>.
3. Compute <code>f(N)</code> = count of good numbers <= <code>N</code> and return <code>f(r) - f(l-1)</code>.

## Solution

```rust
impl Solution { pub fn count_good_integers_on_path(black_l: i64, black_r: i64, black_d: String) -> i64 { let (mut black_p, mut black_r_idx, mut black_c_idx) = ([false; 16], 0, 0); black_p[0] = true; for black_char in black_d.chars() { if black_char == 'D' { black_r_idx += 1; } else { black_c_idx += 1; } black_p[black_r_idx * 4 + black_c_idx] = true; } let (black_ls, black_rs) = (format!("{:016}", black_l), format!("{:016}", black_r)); let (black_lb, black_rb) = (black_ls.as_bytes(), black_rs.as_bytes()); let mut black_m = vec![vec![vec![vec![-1i64; 10]; 2]; 2]; 17]; fn black_dfs(black_i: usize, black_tl: bool, black_tr: bool, black_lv: i32, black_p: &[bool; 16], black_lb: &[u8], black_rb: &[u8], black_m: &mut Vec<Vec<Vec<Vec<i64>>>>) -> i64 { if black_i == 16 { return 1; } if black_m[black_i][black_tl as usize][black_tr as usize][black_lv as usize] != -1 { return black_m[black_i][black_tl as usize][black_tr as usize][black_lv as usize]; } let black_lo = if black_tl { (black_lb[black_i] - b'0') as i32 } else { 0 }; let black_hi = if black_tr { (black_rb[black_i] - b'0') as i32 } else { 9 }; let mut black_ans = 0; for black_digit in black_lo..=black_hi { let (mut black_nv, mut black_ok) = (black_lv, true); if black_p[black_i] { if black_digit < black_lv { black_ok = false; } else { black_nv = black_digit; } } if black_ok { black_ans += black_dfs(black_i + 1, black_tl && black_digit == black_lo, black_tr && black_digit == black_hi, black_nv, black_p, black_lb, black_rb, black_m); } } black_m[black_i][black_tl as usize][black_tr as usize][black_lv as usize] = black_ans; black_ans } black_dfs(0, true, true, 0, &black_p, black_lb, black_rb, &mut black_m) } }
```