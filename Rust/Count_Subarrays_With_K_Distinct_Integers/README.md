# Count Subarrays With K Distinct Integers

**Difficulty:** Hard
**Tags:** Array, Hash Table, Sliding Window, Counting

---

## Problem

<p>You are given an integer array <code>nums</code> and two integers <code>k</code> and <code>m</code>.</p>

<p>Return an integer denoting the count of <strong><span data-keyword="subarray-nonempty">subarrays</span></strong> of <code>nums</code> such that:</p>

<ul>
	<li>The subarray contains <strong>exactly</strong> <code>k</code> <strong>distinct</strong> integers.</li>
	<li>Within the subarray, each <strong>distinct</strong> integer appears <strong>at least</strong> <code>m</code> times.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,1,2,2], k = 2, m = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The possible subarrays with <code>k = 2</code> distinct integers, each appearing at least <code>m = 2</code> times are:</p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Subarray</th>
			<th style="border: 1px solid black;">Distinct<br />
			numbers</th>
			<th style="border: 1px solid black;">Frequency</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">[1, 2, 1, 2]</td>
			<td style="border: 1px solid black;">{1, 2} &rarr; 2</td>
			<td style="border: 1px solid black;">{1: 2, 2: 2}</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">[1, 2, 1, 2, 2]</td>
			<td style="border: 1px solid black;">{1, 2} &rarr; 2</td>
			<td style="border: 1px solid black;">{1: 2, 2: 3}</td>
		</tr>
	</tbody>
</table>

<p>Thus, the answer is 2.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,1,2,4], k = 2, m = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The possible subarrays with <code>k = 2</code> distinct integers, each appearing at least <code>m = 1</code> times are:</p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Subarray</th>
			<th style="border: 1px solid black;">Distinct<br />
			numbers</th>
			<th style="border: 1px solid black;">Frequency</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">[3, 1]</td>
			<td style="border: 1px solid black;">{3, 1} &rarr; 2</td>
			<td style="border: 1px solid black;">{3: 1, 1: 1}</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">[1, 2]</td>
			<td style="border: 1px solid black;">{1, 2} &rarr; 2</td>
			<td style="border: 1px solid black;">{1: 1, 2: 1}</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">[2, 4]</td>
			<td style="border: 1px solid black;">{2, 4} &rarr; 2</td>
			<td style="border: 1px solid black;">{2: 1, 4: 1}</td>
		</tr>
	</tbody>
</table>

<p>Thus, the answer is 3.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k, m &lt;= nums.length</code></li>
</ul>


## Hints

1. Use sliding window.
2. Use the reduction: <code>answer = at_most(k) - at_most(k-1)</code>. <code>at_most(K)</code> = number of subarrays with <code><= K</code> distinct values where every present value appears <code>>= m</code> times.

## Solution

```rust
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32, m: i32) -> i64 {
        let (k, m) = (k as usize, m as usize);
        let n = nums.len();
        let mut black_ans = 0i64;
        let (mut black_f, mut black_valid, mut black_l) = (std::collections::HashMap::new(), 0, 0);
        let (mut black_l2, mut black_f2, mut black_valid2) = (0, std::collections::HashMap::new(), 0);

        for black_r in 0..n {
            let black_v = nums[black_r];
            let black_c = black_f.entry(black_v).or_insert(0);
            *black_c += 1;
            if *black_c == m { black_valid += 1; }

            let black_c2 = black_f2.entry(black_v).or_insert(0);
            *black_c2 += 1;
            if *black_c2 == m { black_valid2 += 1; }

            while black_f.len() > k || (black_f.len() == k && black_valid > k) {
                let lv = nums[black_l];
                let lc = black_f.get_mut(&lv).unwrap();
                if *lc == m { black_valid -= 1; }
                *lc -= 1;
                if *lc == 0 { black_f.remove(&lv); }
                black_l += 1;
            }

            while black_f2.len() > k || (black_f2.len() == k && black_valid2 == k) {
                let lv = nums[black_l2];
                let lc = black_f2.get_mut(&lv).unwrap();
                if *lc == m { black_valid2 -= 1; }
                *lc -= 1;
                if *lc == 0 { black_f2.remove(&lv); }
                black_l2 += 1;
            }

            if black_f.len() == k && black_valid == k {
                black_ans += (black_l2 - black_l) as i64;
            }
        }
        black_ans
    }
}
```