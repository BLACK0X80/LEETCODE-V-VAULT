# Count Sequences to K

**Difficulty:** Hard
**Tags:** Array, Math, Dynamic Programming, Memoization, Number Theory

---

## Problem

<p>You are given an integer array <code>nums</code>, and an integer <code>k</code>.</p>

<p>Start with an initial value <code>val = 1</code> and process <code>nums</code> from left to right. At each index <code>i</code>, you must choose <strong>exactly one</strong> of the following actions:</p>

<ul>
	<li>Multiply <code>val</code> by <code>nums[i]</code>.</li>
	<li>Divide <code>val</code> by <code>nums[i]</code>.</li>
	<li>Leave <code>val</code> unchanged.</li>
</ul>

<p>After processing all elements, <code>val</code> is considered <strong>equal</strong> to <code>k</code> only if its final rational value <strong>exactly</strong> equals <code>k</code>.</p>

<p>Return the count of <strong>distinct</strong> sequences of choices that result in <code>val == k</code>.</p>

<p><strong>Note:</strong> Division is rational (exact), not integer division. For example, <code>2 / 4 = 1 / 2</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,3,2], k = 6</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The following 2 distinct sequences of choices result in <code>val == k</code>:</p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Sequence</th>
			<th style="border: 1px solid black;">Operation on <code>nums[0]</code></th>
			<th style="border: 1px solid black;">Operation on <code>nums[1]</code></th>
			<th style="border: 1px solid black;">Operation on <code>nums[2]</code></th>
			<th style="border: 1px solid black;">Final <code>val</code></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">Multiply: <code>val = 1 * 2 = 2</code></td>
			<td style="border: 1px solid black;">Multiply: <code>val = 2 * 3 = 6</code></td>
			<td style="border: 1px solid black;">Leave <code>val</code> unchanged</td>
			<td style="border: 1px solid black;">6</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">Leave <code>val</code> unchanged</td>
			<td style="border: 1px solid black;">Multiply: <code>val = 1 * 3 = 3</code></td>
			<td style="border: 1px solid black;">Multiply: <code>val = 3 * 2 = 6</code></td>
			<td style="border: 1px solid black;">6</td>
		</tr>
	</tbody>
</table>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,6,3], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The following 2 distinct sequences of choices result in <code>val == k</code>:</p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Sequence</th>
			<th style="border: 1px solid black;">Operation on <code>nums[0]</code></th>
			<th style="border: 1px solid black;">Operation on <code>nums[1]</code></th>
			<th style="border: 1px solid black;">Operation on <code>nums[2]</code></th>
			<th style="border: 1px solid black;">Final <code>val</code></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">Multiply: <code>val = 1 * 4 = 4</code></td>
			<td style="border: 1px solid black;">Divide: <code>val = 4 / 6 = 2 / 3</code></td>
			<td style="border: 1px solid black;">Multiply: <code>val = (2 / 3) * 3 = 2</code></td>
			<td style="border: 1px solid black;">2</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">Leave <code>val</code> unchanged</td>
			<td style="border: 1px solid black;">Multiply: <code>val = 1 * 6 = 6</code></td>
			<td style="border: 1px solid black;">Divide: <code>val = 6 / 3 = 2</code></td>
			<td style="border: 1px solid black;">2</td>
		</tr>
	</tbody>
</table>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,5], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The following 3 distinct sequences of choices result in <code>val == k</code>:</p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Sequence</th>
			<th style="border: 1px solid black;">Operation on <code>nums[0]</code></th>
			<th style="border: 1px solid black;">Operation on <code>nums[1]</code></th>
			<th style="border: 1px solid black;">Final <code>val</code></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">Multiply: <code>val = 1 * 1 = 1</code></td>
			<td style="border: 1px solid black;">Leave <code>val</code> unchanged</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">Divide: <code>val = 1 / 1 = 1</code></td>
			<td style="border: 1px solid black;">Leave <code>val</code> unchanged</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">Leave <code>val</code> unchanged</td>
			<td style="border: 1px solid black;">Leave <code>val</code> unchanged</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
	</tbody>
</table>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 19</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 6</code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>15</sup></code></li>
</ul>


## Hints

1. Represent numbers by their prime exponents as <code>(2^x, 3^y, 5^z)</code>.
2. Use Dynamic Programming on the prime exponents: let <code>dp(idx, x, y, z)</code> be the number of ways to reach exponents <code>(x,y,z)</code> after processing index <code>idx</code>.
3. When <code>idx == nums.length</code>, compare <code>(x, y, z)</code> to the prime exponent decomposition of <code>k</code> and count matches.

## Solution

```rust
use std::collections::HashMap;
impl Solution {
    pub fn count_sequences(black1: Vec<i32>, black2: i64) -> i32 {
        fn black3(black4: i64) -> (i8, i8, i8, i64) {
            let (mut black5, mut black6, mut black7, mut black8) = (0, 0, 0, black4);
            while black8 > 0 && black8 % 2 == 0 { black5 += 1; black8 /= 2; }
            while black8 > 0 && black8 % 3 == 0 { black6 += 1; black8 /= 3; }
            while black8 > 0 && black8 % 5 == 0 { black7 += 1; black8 /= 5; }
            (black5, black6, black7, black8)
        }
        let (black9, black10, black11, black12) = black3(black2);
        if black12 != 1 { return 0; }
        let mut black13: HashMap<(i8, i8, i8), i32> = HashMap::from([((0, 0, 0), 1)]);
        for black14 in black1 {
            let (black15, black16, black17, _) = black3(black14 as i64);
            let mut black18 = HashMap::new();
            for (&(black19, black20, black21), &black22) in &black13 {
                for &(black23, black24, black25) in &[(black15, black16, black17), (-black15, -black16, -black17), (0, 0, 0)] {
                    *black18.entry((black19 + black23, black20 + black24, black21 + black25)).or_insert(0) += black22;
                }
            }
            black13 = black18;
        }
        *black13.get(&(black9, black10, black11)).unwrap_or(&0)
    }
}
```