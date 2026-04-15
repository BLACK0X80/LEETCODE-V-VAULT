# Number of Integers With Popcount-Depth Equal to K I

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming, Bit Manipulation, Combinatorics

---

## Problem

<p>You are given two integers <code>n</code> and <code>k</code>.</p>

<p>For any positive integer <code>x</code>, define the following sequence:</p>

<ul>
	<li><code>p<sub>0</sub> = x</code></li>
	<li><code>p<sub>i+1</sub> = popcount(p<sub>i</sub>)</code> for all <code>i &gt;= 0</code>, where <code>popcount(y)</code> is the number of set bits (1&#39;s) in the binary representation of <code>y</code>.</li>
</ul>

<p>This sequence will eventually reach the value 1.</p>

<p>The <strong>popcount-depth</strong> of <code>x</code> is defined as the <strong>smallest</strong> integer <code>d &gt;= 0</code> such that <code>p<sub>d</sub> = 1</code>.</p>

<p>For example, if <code>x = 7</code> (binary representation <code>&quot;111&quot;</code>). Then, the sequence is: <code>7 &rarr; 3 &rarr; 2 &rarr; 1</code>, so the popcount-depth of 7 is 3.</p>

<p>Your task is to determine the number of integers in the range <code>[1, n]</code> whose popcount-depth is <strong>exactly</strong> equal to <code>k</code>.</p>

<p>Return the number of such integers.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4, k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The following integers in the range <code>[1, 4]</code> have popcount-depth exactly equal to 1:</p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th align="center" style="border: 1px solid black;">x</th>
			<th align="center" style="border: 1px solid black;">Binary</th>
			<th align="left" style="border: 1px solid black;">Sequence</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td align="center" style="border: 1px solid black;">2</td>
			<td align="center" style="border: 1px solid black;"><code>&quot;10&quot;</code></td>
			<td align="left" style="border: 1px solid black;"><code>2 &rarr; 1</code></td>
		</tr>
		<tr>
			<td align="center" style="border: 1px solid black;">4</td>
			<td align="center" style="border: 1px solid black;"><code>&quot;100&quot;</code></td>
			<td align="left" style="border: 1px solid black;"><code>4 &rarr; 1</code></td>
		</tr>
	</tbody>
</table>

<p>Thus, the answer is 2.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 7, k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The following integers in the range <code>[1, 7]</code> have popcount-depth exactly equal to 2:</p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">x</th>
			<th style="border: 1px solid black;">Binary</th>
			<th style="border: 1px solid black;">Sequence</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;"><code>&quot;11&quot;</code></td>
			<td style="border: 1px solid black;"><code>3 &rarr; 2 &rarr; 1</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">5</td>
			<td style="border: 1px solid black;"><code>&quot;101&quot;</code></td>
			<td style="border: 1px solid black;"><code>5 &rarr; 2 &rarr; 1</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">6</td>
			<td style="border: 1px solid black;"><code>&quot;110&quot;</code></td>
			<td style="border: 1px solid black;"><code>6 &rarr; 2 &rarr; 1</code></td>
		</tr>
	</tbody>
</table>

<p>Thus, the answer is 3.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>15</sup></code></li>
	<li><code>0 &lt;= k &lt;= 5</code></li>
</ul>


## Hints

1. Use digit dynamic programming on the binary representation of <code>n</code>: let <code>dp[pos][ones][tight]</code> = number of ways to choose bits from the most significant down to position <code>pos</code> with exactly <code>ones</code> ones so far, where <code>tight</code> indicates whether you're still matching the prefix of <code>n</code>.
2. Precompute <code>depth[j]</code> for all <code>j</code> from <code>0</code> to <code>64</code> by repeatedly applying <code>popcount(j)</code> until you reach <code>1</code>.
3. After your DP, let <code>dp_final[j]</code> be the count of numbers <= <code>n</code> that have exactly <code>j</code> ones; the answer is the sum of all <code>dp_final[j]</code> for which <code>depth[j] == k</code>.

## Solution

```rust
impl Solution {
    pub fn popcount_depth(black_n: i64, black_k: i32) -> i64 {
        if black_k == 0 { return if black_n >= 1 { 1 } else { 0 }; }
        
        let mut black_depth = vec![0; 65];
        black_depth[1] = 0;
        for i in 2..65 {
            black_depth[i] = 1 + black_depth[i.count_ones() as usize];
        }

        let mut black_comb = vec![vec![0i64; 65]; 65];
        for i in 0..65 {
            black_comb[i][0] = 1;
            for j in 1..=i {
                black_comb[i][j] = black_comb[i - 1][j - 1] + black_comb[i - 1][j];
            }
        }

        let mut black_ans = 0;
        let mut black_set_bits = 0;
        let black_bits: Vec<i32> = format!("{:b}", black_n).chars().map(|c| c.to_digit(2).unwrap() as i32).collect();
        let black_len = black_bits.len();

        for i in 0..black_len {
            if black_bits[i] == 1 {
                let black_rem = black_len - 1 - i;
                for b in 0..=black_rem {
                    let black_total = black_set_bits + b;
                    if black_total > 0 && black_depth[black_total] == black_k - 1 {
                        black_ans += black_comb[black_rem][b];
                    }
                }
                black_set_bits += 1;
            }
        }

        if black_depth[black_set_bits as usize] == black_k - 1 {
            black_ans += 1;
        }

        if black_k == 1 { black_ans -= 1; }
        black_ans
    }
}
```