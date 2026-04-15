# Count Ways to Choose Coprime Integers from Rows

**Difficulty:** Hard
**Tags:** Array, Math, Dynamic Programming, Matrix, Combinatorics, Number Theory

---

## Problem

<p>You are given a <code>m x n</code> matrix <code>mat</code> of positive integers.</p>

<p>Return an integer denoting the number of ways to choose <strong>exactly one</strong> integer from each row of <code>mat</code> such that the <strong>greatest common divisor</strong> of all chosen integers is 1.</p>

<p>Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">mat = [[1,2],[3,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<tbody>
		<tr>
			<th align="center" style="border: 1px solid black;">Chosen integer in the first row</th>
			<th align="center" style="border: 1px solid black;">Chosen integer in the second row</th>
			<th align="center" style="border: 1px solid black;">Greatest common divisor of chosen integers</th>
		</tr>
		<tr>
			<td align="center" style="border: 1px solid black;">1</td>
			<td align="center" style="border: 1px solid black;">3</td>
			<td align="center" style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td align="center" style="border: 1px solid black;">1</td>
			<td align="center" style="border: 1px solid black;">4</td>
			<td align="center" style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td align="center" style="border: 1px solid black;">2</td>
			<td align="center" style="border: 1px solid black;">3</td>
			<td align="center" style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td align="center" style="border: 1px solid black;">2</td>
			<td align="center" style="border: 1px solid black;">4</td>
			<td align="center" style="border: 1px solid black;">2</td>
		</tr>
	</tbody>
</table>

<p>3 of these combinations have a greatest common divisor of 1. Therefore, the answer is 3.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">mat = [[2,2],[2,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>Every combination has a greatest common divisor of 2. Therefore, the answer is 0.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m == mat.length &lt;= 150</code></li>
	<li><code>1 &lt;= n == mat[i].length &lt;= 150</code></li>
	<li><code>1 &lt;= mat[i][j] &lt;= 150</code></li>
</ul>


## Hints

1. Use dynamic programming.
2. Use <code>dp[row][g]</code>, where <code>row</code> is the current row and <code>g</code> is the current gcd value.
3. Initialize the first row: for each value <code>v</code> in row 0 do <code>dp[0][v] += 1</code>.
4. For a row <code>i</code>, use the values from the previous row <code>i - 1</code> to build the values: for each previous gcd <code>g</code> and each value <code>v</code> in row <code>i</code>.
5. The final answer is <code>dp[n-1][1]</code> (number of ways with gcd 1).

## Solution

```rust
impl Solution {
    pub fn count_coprime(black_mat: Vec<Vec<i32>>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_m = black_mat.len();
        let black_max_val = 150;

        let mut black_mu = vec![0; black_max_val + 1];
        black_mu[1] = 1;
        let mut black_primes = vec![];
        let mut black_is_prime = vec![true; black_max_val + 1];
        for i in 2..=black_max_val {
            if black_is_prime[i] {
                black_primes.push(i);
                black_mu[i] = -1;
            }
            for &p in &black_primes {
                if i * p > black_max_val { break; }
                black_is_prime[i * p] = false;
                if i % p == 0 { black_mu[i * p] = 0; break; }
                else { black_mu[i * p] = -black_mu[i]; }
            }
        }

        let mut black_ans = 0i64;
        for g in 1..=black_max_val {
            if black_mu[g] == 0 { continue; }
            
            let mut black_ways_for_g = 1i64;
            for row in &black_mat {
                let mut black_count_in_row = 0i64;
                for &val in row {
                    if val % g as i32 == 0 { black_count_in_row += 1; }
                }
                black_ways_for_g = (black_ways_for_g * black_count_in_row) % black_mod;
            }

            if black_mu[g] == 1 {
                black_ans = (black_ans + black_ways_for_g) % black_mod;
            } else {
                black_ans = (black_ans - black_ways_for_g + black_mod) % black_mod;
            }
        }

        black_ans as i32
    }
}
```