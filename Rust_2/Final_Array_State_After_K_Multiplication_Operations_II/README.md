# Final Array State After K Multiplication Operations II

**Difficulty:** Hard
**Tags:** Array, Heap (Priority Queue), Simulation

---

## Problem

<p>You are given an integer array <code>nums</code>, an integer <code>k</code>, and an integer <code>multiplier</code>.</p>

<p>You need to perform <code>k</code> operations on <code>nums</code>. In each operation:</p>

<ul>
	<li>Find the <strong>minimum</strong> value <code>x</code> in <code>nums</code>. If there are multiple occurrences of the minimum value, select the one that appears <strong>first</strong>.</li>
	<li>Replace the selected minimum value <code>x</code> with <code>x * multiplier</code>.</li>
</ul>

<p>After the <code>k</code> operations, apply <strong>modulo</strong> <code>10<sup>9</sup> + 7</code> to every value in <code>nums</code>.</p>

<p>Return an integer array denoting the <em>final state</em> of <code>nums</code> after performing all <code>k</code> operations and then applying the modulo.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,1,3,5,6], k = 5, multiplier = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">[8,4,6,5,6]</span></p>

<p><strong>Explanation:</strong></p>

<table>
	<tbody>
		<tr>
			<th>Operation</th>
			<th>Result</th>
		</tr>
		<tr>
			<td>After operation 1</td>
			<td>[2, 2, 3, 5, 6]</td>
		</tr>
		<tr>
			<td>After operation 2</td>
			<td>[4, 2, 3, 5, 6]</td>
		</tr>
		<tr>
			<td>After operation 3</td>
			<td>[4, 4, 3, 5, 6]</td>
		</tr>
		<tr>
			<td>After operation 4</td>
			<td>[4, 4, 6, 5, 6]</td>
		</tr>
		<tr>
			<td>After operation 5</td>
			<td>[8, 4, 6, 5, 6]</td>
		</tr>
		<tr>
			<td>After applying modulo</td>
			<td>[8, 4, 6, 5, 6]</td>
		</tr>
	</tbody>
</table>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [100000,2000], k = 2, multiplier = 1000000</span></p>

<p><strong>Output:</strong> <span class="example-io">[999999307,999999993]</span></p>

<p><strong>Explanation:</strong></p>

<table>
	<tbody>
		<tr>
			<th>Operation</th>
			<th>Result</th>
		</tr>
		<tr>
			<td>After operation 1</td>
			<td>[100000, 2000000000]</td>
		</tr>
		<tr>
			<td>After operation 2</td>
			<td>[100000000000, 2000000000]</td>
		</tr>
		<tr>
			<td>After applying modulo</td>
			<td>[999999307, 999999993]</td>
		</tr>
	</tbody>
</table>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= multiplier &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. What happens when <code>min(nums) * multiplier > max(nums)</code>?
2. A cycle of operations begins.
3. Simulate until <code>min(nums) * multiplier > max(nums)</code>, then greedily distribute remaining multiplications.

## Solution

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn get_final_state(mut black1: Vec<i32>, mut black2: i32, black3: i32) -> Vec<i32> {
        if black3 == 1 { return black1; }
        let black4 = 1_000_000_007i64;
        let mut black5 = BinaryHeap::new();
        for (black6, &black7) in black1.iter().enumerate() {
            black5.push(Reverse((black7 as i64, black6)));
        }
        let black8 = *black1.iter().max().unwrap() as i64;
        while black2 > 0 && black5.peek().unwrap().0.0 * black3 as i64 <= black8 {
            let Reverse((mut black9, black10)) = black5.pop().unwrap();
            black9 *= black3 as i64;
            black5.push(Reverse((black9, black10)));
            black2 -= 1;
        }
        let black11 = (black2 / black1.len() as i32) as i64;
        let black12 = black2 % black1.len() as i32;
        let mut black13 = |mut black14: i64, mut black15: i64| {
            let mut black16 = 1;
            black14 %= black4;
            while black15 > 0 {
                if black15 % 2 == 1 { black16 = (black16 * black14) % black4; }
                black14 = (black14 * black14) % black4;
                black15 /= 2;
            }
            black16
        };
        let black17 = black13(black3 as i64, black11);
        let black18 = (black17 * black3 as i64) % black4;
        let mut black19 = vec![0; black1.len()];
        for black20 in 0..black1.len() {
            let Reverse((black21, black22)) = black5.pop().unwrap();
            let black23 = if black20 < black12 as usize { black18 } else { black17 };
            black19[black22] = ((black21 % black4 * black23) % black4) as i32;
        }
        black19
    }
}
```