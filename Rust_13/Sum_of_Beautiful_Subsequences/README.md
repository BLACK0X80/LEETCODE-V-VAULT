# Sum of Beautiful Subsequences

**Difficulty:** Hard
**Tags:** Array, Math, Binary Indexed Tree, Number Theory

---

## Problem

<p>You are given an integer array <code>nums</code> of length <code>n</code>.</p>

<p>For every <strong>positive</strong> integer <code>g</code>, we define the <strong>beauty</strong> of <code>g</code> as the <strong>product</strong> of <code>g</code> and the number of <strong>strictly increasing</strong> <strong><span data-keyword="subsequence-array-nonempty">subsequences</span></strong> of <code>nums</code> whose greatest common divisor (GCD) is exactly <code>g</code>.</p>

<p>Return the <strong>sum</strong> of <strong>beauty</strong> values for all positive integers <code>g</code>.</p>

<p>Since the answer could be very large, return it modulo <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">10</span></p>

<p><strong>Explanation:</strong></p>

<p>All strictly increasing subsequences and their GCDs are:</p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Subsequence</th>
			<th style="border: 1px solid black;">GCD</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">[1]</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">[2]</td>
			<td style="border: 1px solid black;">2</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">[3]</td>
			<td style="border: 1px solid black;">3</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">[1,2]</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">[1,3]</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">[2,3]</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">[1,2,3]</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
	</tbody>
</table>

<p>Calculating beauty for each GCD:</p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">GCD</th>
			<th style="border: 1px solid black;">Count of subsequences</th>
			<th style="border: 1px solid black;">Beauty (GCD &times; Count)</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">5</td>
			<td style="border: 1px solid black;">1 &times; 5 = 5</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">2 &times; 1 = 2</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">3 &times; 1 = 3</td>
		</tr>
	</tbody>
</table>

<p>Total beauty is <code>5 + 2 + 3 = 10</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,6]</span></p>

<p><strong>Output:</strong> <span class="example-io">12</span></p>

<p><strong>Explanation:</strong></p>

<p>All strictly increasing subsequences and their GCDs are:</p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Subsequence</th>
			<th style="border: 1px solid black;">GCD</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">[4]</td>
			<td style="border: 1px solid black;">4</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">[6]</td>
			<td style="border: 1px solid black;">6</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">[4,6]</td>
			<td style="border: 1px solid black;">2</td>
		</tr>
	</tbody>
</table>

<p>Calculating beauty for each GCD:</p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">GCD</th>
			<th style="border: 1px solid black;">Count of subsequences</th>
			<th style="border: 1px solid black;">Beauty (GCD &times; Count)</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">2 &times; 1 = 2</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">4</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">4 &times; 1 = 4</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">6</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">6 &times; 1 = 6</td>
		</tr>
	</tbody>
</table>

<p>Total beauty is <code>2 + 4 + 6 = 12</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 7 * 10<sup>4</sup></code></li>
</ul>


## Hints

1. Fix a candidate GCD <code>g</code> and keep, in the original order, only those array elements divisible by <code>g</code>; scale them down to <code>x / g</code> so any increasing subsequence here corresponds to a subsequence whose elements are all multiples of <code>g</code>.
2. Count strictly increasing subsequences of that scaled list by assigning ranks (coordinate compression) and maintaining prefix sums of ways for smaller ranks (you may use a Fenwick tree).
3. The count you get, call it <code>cnt_g</code>, includes subsequences whose GCD is <code>g</code> or any multiple of <code>g</code>; it is therefore an overcount for "exactly g".
4. To get the number with GCD exactly <code>g</code>, process <code>g</code> from <code>max(nums)</code> down to <code>1</code> and subtract counts already assigned to multiples: <code>F[g] = cnt_g - sum{k=2g,3g,...}*F[k]</code> (do arithmetic mod <code>MOD</code>); descending order ensures multiples are known.
5. The final answer is the sum of contributions <code>g * F[g]</code> for all <code>g</code>.

## Solution

```rust
impl Solution {
    pub fn total_beauty(black_nums: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_max_a = *black_nums.iter().max().unwrap_or(&0) as usize;
        
        let mut black_divisors = vec![Vec::new(); black_max_a + 1];
        for black_d in 1..=black_max_a {
            for black_m in (black_d..=black_max_a).step_by(black_d) {
                black_divisors[black_m].push(black_d);
            }
        }

        let mut black_a = vec![Vec::new(); black_max_a + 1];
        for &black_x in &black_nums {
            for &black_d in &black_divisors[black_x as usize] {
                black_a[black_d].push(black_x);
            }
        }

        let mut black_num_inc = vec![0i64; black_max_a + 1];
        for black_x in 1..=black_max_a {
            if !black_a[black_x].is_empty() {
                black_num_inc[black_x] = Self::black_count_increasing(&black_a[black_x], black_mod);
            }
        }

        let mut black_dp = vec![0i64; black_max_a + 1];
        for black_x in (1..=black_max_a).rev() {
            let mut black_v = black_num_inc[black_x];
            for black_y in (black_x * 2..=black_max_a).step_by(black_x) {
                black_v = (black_v - black_dp[black_y] + black_mod) % black_mod;
            }
            black_dp[black_x] = black_v;
        }

        let mut black_ans = 0i64;
        for black_x in 1..=black_max_a {
            if black_dp[black_x] > 0 {
                black_ans = (black_ans + (black_x as i64 * black_dp[black_x])) % black_mod;
            }
        }
        black_ans as i32
    }

    fn black_count_increasing(black_seq: &Vec<i32>, black_mod: i64) -> i64 {
        let mut black_vals = black_seq.clone();
        black_vals.sort_unstable();
        black_vals.dedup();
        
        let black_m = black_vals.len();
        let mut black_bit = vec![0i64; black_m + 1];

        let mut black_total = 0i64;
        for &black_v in black_seq {
            let black_r = black_vals.binary_search(&black_v).unwrap() + 1;
            
            let mut black_less = 0i64;
            let mut black_idx = black_r - 1;
            while black_idx > 0 {
                black_less = (black_less + black_bit[black_idx]) % black_mod;
                black_idx -= (black_idx as i32 & -(black_idx as i32)) as usize;
            }

            let black_add_here = (black_less + 1) % black_mod;
            
            let mut black_update_idx = black_r;
            while black_update_idx <= black_m {
                black_bit[black_update_idx] = (black_bit[black_update_idx] + black_add_here) % black_mod;
                black_update_idx += (black_update_idx as i32 & -(black_update_idx as i32)) as usize;
            }
            
            black_total = (black_total + black_add_here) % black_mod;
        }
        black_total
    }
}
```