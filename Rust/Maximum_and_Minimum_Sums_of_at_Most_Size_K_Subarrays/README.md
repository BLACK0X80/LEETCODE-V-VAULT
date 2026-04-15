# Maximum and Minimum Sums of at Most Size K Subarrays

**Difficulty:** Hard
**Tags:** Array, Math, Stack, Monotonic Stack

---

## Problem

<p>You are given an integer array <code>nums</code> and a <strong>positive</strong> integer <code>k</code>. Return the sum of the <strong>maximum</strong> and <strong>minimum</strong> elements of all <span data-keyword="subarray-nonempty">subarrays</span> with <strong>at most</strong> <code>k</code> elements.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">20</span></p>

<p><strong>Explanation:</strong></p>

<p>The subarrays of <code>nums</code> with at most 2 elements are:</p>

<table style="border: 1px solid black;">
	<tbody>
		<tr>
			<th style="border: 1px solid black;"><b>Subarray</b></th>
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
			<td style="border: 1px solid black;"><code>[2, 3]</code></td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">5</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><strong>Final Total</strong></td>
			<td style="border: 1px solid black;">&nbsp;</td>
			<td style="border: 1px solid black;">&nbsp;</td>
			<td style="border: 1px solid black;">20</td>
		</tr>
	</tbody>
</table>

<p>The output would be 20.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,-3,1], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">-6</span></p>

<p><strong>Explanation:</strong></p>

<p>The subarrays of <code>nums</code> with at most 2 elements are:</p>

<table style="border: 1px solid black;">
	<tbody>
		<tr>
			<th style="border: 1px solid black;"><b>Subarray</b></th>
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
			<td style="border: 1px solid black;"><code>[-3]</code></td>
			<td style="border: 1px solid black;">-3</td>
			<td style="border: 1px solid black;">-3</td>
			<td style="border: 1px solid black;">-6</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[1]</code></td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">2</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[1, -3]</code></td>
			<td style="border: 1px solid black;">-3</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">-2</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[-3, 1]</code></td>
			<td style="border: 1px solid black;">-3</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">-2</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><strong>Final Total</strong></td>
			<td style="border: 1px solid black;">&nbsp;</td>
			<td style="border: 1px solid black;">&nbsp;</td>
			<td style="border: 1px solid black;">-6</td>
		</tr>
	</tbody>
</table>

<p>The output would be -6.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 80000</code></li>
	<li><code>1 &lt;= k &lt;= nums.length</code></li>
	<li><code>-10<sup>6</sup> &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Use a monotonic stack.
2. How can we calculate the number of subarrays where an element is the largest?
3. Enforce the condition on size too.

## Solution

```rust
impl Solution {
    pub fn min_max_subarray_sum(black_nums: Vec<i32>, black_k: i32) -> i64 {
        let black_n = black_nums.len();
        let black_k = black_k as i64;
        let mut black_ans: i64 = 0;

        fn black_solve(black_arr: &[i32], black_n: usize, black_k: i64, black_is_max: bool) -> i64 {
            let mut black_left = vec![0i64; black_n];
            let mut black_right = vec![0i64; black_n];
            let mut black_stack: Vec<usize> = Vec::with_capacity(black_n);

            for black_i in 0..black_n {
                while let Some(&black_top) = black_stack.last() {
                    let black_cond = if black_is_max {
                        black_arr[black_top] < black_arr[black_i]
                    } else {
                        black_arr[black_top] > black_arr[black_i]
                    };
                    if black_cond {
                        black_stack.pop();
                    } else {
                        break;
                    }
                }
                black_left[black_i] = if black_stack.is_empty() {
                    black_i as i64
                } else {
                    (black_i - black_stack.last().unwrap() - 1) as i64
                };
                black_stack.push(black_i);
            }

            black_stack.clear();
            for black_i in (0..black_n).rev() {
                while let Some(&black_top) = black_stack.last() {
                    let black_cond = if black_is_max {
                        black_arr[black_top] <= black_arr[black_i]
                    } else {
                        black_arr[black_top] >= black_arr[black_i]
                    };
                    if black_cond {
                        black_stack.pop();
                    } else {
                        break;
                    }
                }
                black_right[black_i] = if black_stack.is_empty() {
                    (black_n - 1 - black_i) as i64
                } else {
                    (black_stack.last().unwrap() - black_i - 1) as i64
                };
                black_stack.push(black_i);
            }

            let mut black_res: i64 = 0;
            for black_i in 0..black_n {
                black_res += black_arr[black_i] as i64 * black_count(black_left[black_i], black_right[black_i], black_k);
            }
            black_res
        }

        fn black_count(black_l: i64, black_r: i64, black_k: i64) -> i64 {
            let black_m = black_k - 1;
            let black_s1 = black_l.min(black_m);
            if black_m - black_r > black_s1 {
                (black_s1 + 1) * (black_r + 1)
            } else {
                let black_split = 0.max(black_m - black_r);
                let black_p1 = black_split * (black_r + 1);
                let black_num_terms = black_s1 - black_split + 1;
                let black_first = black_m - black_split + 1;
                let black_last = black_m - black_s1 + 1;
                let black_p2 = (black_num_terms * (black_first + black_last)) / 2;
                black_p1 + black_p2
            }
        }

        black_ans += black_solve(&black_nums, black_n, black_k, true);
        black_ans += black_solve(&black_nums, black_n, black_k, false);
        black_ans
    }
}
```