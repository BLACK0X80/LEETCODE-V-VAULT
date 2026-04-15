# Make Array Empty

**Difficulty:** Hard
**Tags:** Array, Binary Search, Greedy, Binary Indexed Tree, Segment Tree, Sorting, Ordered Set

---

## Problem

<p>You are given an integer array <code>nums</code> containing <strong>distinct</strong> numbers, and you can perform the following operations <strong>until the array is empty</strong>:</p>

<ul>
	<li>If the first element has the <strong>smallest</strong> value, remove it</li>
	<li>Otherwise, put the first element at the <strong>end</strong> of the array.</li>
</ul>

<p>Return <em>an integer denoting the number of operations it takes to make </em><code>nums</code><em> empty.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,4,-1]
<strong>Output:</strong> 5
</pre>

<table style="border: 2px solid black; border-collapse: collapse;">
	<thead>
		<tr>
			<th style="border: 2px solid black; padding: 5px;">Operation</th>
			<th style="border: 2px solid black; padding: 5px;">Array</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 2px solid black; padding: 5px;">1</td>
			<td style="border: 2px solid black; padding: 5px;">[4, -1, 3]</td>
		</tr>
		<tr>
			<td style="border: 2px solid black; padding: 5px;">2</td>
			<td style="border: 2px solid black; padding: 5px;">[-1, 3, 4]</td>
		</tr>
		<tr>
			<td style="border: 2px solid black; padding: 5px;">3</td>
			<td style="border: 2px solid black; padding: 5px;">[3, 4]</td>
		</tr>
		<tr>
			<td style="border: 2px solid black; padding: 5px;">4</td>
			<td style="border: 2px solid black; padding: 5px;">[4]</td>
		</tr>
		<tr>
			<td style="border: 2px solid black; padding: 5px;">5</td>
			<td style="border: 2px solid black; padding: 5px;">[]</td>
		</tr>
	</tbody>
</table>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,4,3]
<strong>Output:</strong> 5
</pre>

<table style="border: 2px solid black; border-collapse: collapse;">
	<thead>
		<tr>
			<th style="border: 2px solid black; padding: 5px;">Operation</th>
			<th style="border: 2px solid black; padding: 5px;">Array</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 2px solid black; padding: 5px;">1</td>
			<td style="border: 2px solid black; padding: 5px;">[2, 4, 3]</td>
		</tr>
		<tr>
			<td style="border: 2px solid black; padding: 5px;">2</td>
			<td style="border: 2px solid black; padding: 5px;">[4, 3]</td>
		</tr>
		<tr>
			<td style="border: 2px solid black; padding: 5px;">3</td>
			<td style="border: 2px solid black; padding: 5px;">[3, 4]</td>
		</tr>
		<tr>
			<td style="border: 2px solid black; padding: 5px;">4</td>
			<td style="border: 2px solid black; padding: 5px;">[4]</td>
		</tr>
		<tr>
			<td style="border: 2px solid black; padding: 5px;">5</td>
			<td style="border: 2px solid black; padding: 5px;">[]</td>
		</tr>
	</tbody>
</table>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> 3
</pre>

<table style="border: 2px solid black; border-collapse: collapse;">
	<thead>
		<tr>
			<th style="border: 2px solid black; padding: 5px;">Operation</th>
			<th style="border: 2px solid black; padding: 5px;">Array</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 2px solid black; padding: 5px;">1</td>
			<td style="border: 2px solid black; padding: 5px;">[2, 3]</td>
		</tr>
		<tr>
			<td style="border: 2px solid black; padding: 5px;">2</td>
			<td style="border: 2px solid black; padding: 5px;">[3]</td>
		</tr>
		<tr>
			<td style="border: 2px solid black; padding: 5px;">3</td>
			<td style="border: 2px solid black; padding: 5px;">[]</td>
		</tr>
	</tbody>
</table>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>9&nbsp;</sup>&lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li>All values in <code>nums</code> are <strong>distinct</strong>.</li>
</ul>


## Hints

1. Understand the order in which the indices are removed from the array.
2. We don’t really need to delete or move the elements, only the array length matters.
3. Upon removing an index, decide how many steps it takes to move to the next one.
4. Use a data structure to speed up the calculation.

## Solution

```rust
impl Solution {
    pub fn count_operations_to_empty_array(nums: Vec<i32>) -> i64 {
        let black_n = nums.len();
        let mut black_sorted_indices: Vec<usize> = (0..black_n).collect();
        black_sorted_indices.sort_by_key(|&i| nums[i]);

        let mut black_bit = vec![0; black_n + 1];
        for i in 1..=black_n {
            let mut black_idx = i;
            while black_idx <= black_n {
                black_bit[black_idx] += 1;
                black_idx += (black_idx as i32 & -(black_idx as i32)) as usize;
            }
        }

        fn black_query(black_bit: &Vec<i32>, mut black_idx: usize) -> i32 {
            let mut black_s = 0;
            while black_idx > 0 {
                black_s += black_bit[black_idx];
                black_idx -= (black_idx as i32 & -(black_idx as i32)) as usize;
            }
            black_s
        }

        let mut black_ans = 0i64;
        let mut black_curr = 0;
        for &black_next in &black_sorted_indices {
            if black_next >= black_curr {
                black_ans += (black_query(&black_bit, black_next + 1) - black_query(&black_bit, black_curr)) as i64;
            } else {
                black_ans += (black_query(&black_bit, black_n) - black_query(&black_bit, black_curr) + black_query(&black_bit, black_next + 1)) as i64;
            }
            let mut black_u = black_next + 1;
            while black_u <= black_n {
                black_bit[black_u] -= 1;
                black_u += (black_u as i32 & -(black_u as i32)) as usize;
            }
            black_curr = black_next;
        }
        black_ans
    }
}
```