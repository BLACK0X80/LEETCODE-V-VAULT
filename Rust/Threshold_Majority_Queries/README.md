# Threshold Majority Queries

**Difficulty:** Hard
**Tags:** Array, Hash Table, Binary Search, Divide and Conquer, Counting, Prefix Sum

---

## Problem

<p>You are given an integer array <code>nums</code> of length <code>n</code> and an array <code>queries</code>, where <code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>, threshold<sub>i</sub>]</code>.</p>

<p>Return an array of integers <code data-end="33" data-start="28">ans</code> where <code data-end="48" data-start="40">ans[i]</code> is equal to the element in the subarray <code data-end="102" data-start="89">nums[l<sub>i</sub>...r<sub>i</sub>]</code> that appears <strong>at least</strong> <code data-end="137" data-start="125">threshold<sub>i</sub></code> times, selecting the element with the <strong>highest</strong> frequency (choosing the <strong>smallest</strong> in case of a tie), or -1 if no such element <em>exists</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1,2,2,1,1], queries = [[0,5,4],[0,3,3],[2,3,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,-1,2]</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th align="left" style="border: 1px solid black;">Query</th>
			<th align="left" style="border: 1px solid black;">Sub-array</th>
			<th align="left" style="border: 1px solid black;">Threshold</th>
			<th align="left" style="border: 1px solid black;">Frequency table</th>
			<th align="left" style="border: 1px solid black;">Answer</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td align="left" style="border: 1px solid black;">[0, 5, 4]</td>
			<td align="left" style="border: 1px solid black;">[1, 1, 2, 2, 1, 1]</td>
			<td align="left" style="border: 1px solid black;">4</td>
			<td align="left" style="border: 1px solid black;">1 &rarr; 4, 2 &rarr; 2</td>
			<td align="left" style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td align="left" style="border: 1px solid black;">[0, 3, 3]</td>
			<td align="left" style="border: 1px solid black;">[1, 1, 2, 2]</td>
			<td align="left" style="border: 1px solid black;">3</td>
			<td align="left" style="border: 1px solid black;">1 &rarr; 2, 2 &rarr; 2</td>
			<td align="left" style="border: 1px solid black;">-1</td>
		</tr>
		<tr>
			<td align="left" style="border: 1px solid black;">[2, 3, 2]</td>
			<td align="left" style="border: 1px solid black;">[2, 2]</td>
			<td align="left" style="border: 1px solid black;">2</td>
			<td align="left" style="border: 1px solid black;">2 &rarr; 2</td>
			<td align="left" style="border: 1px solid black;">2</td>
		</tr>
	</tbody>
</table>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,2,3,2,3,2,3], queries = [[0,6,4],[1,5,2],[2,4,1],[3,3,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[3,2,3,2]</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th align="left" style="border: 1px solid black;">Query</th>
			<th align="left" style="border: 1px solid black;">Sub-array</th>
			<th align="left" style="border: 1px solid black;">Threshold</th>
			<th align="left" style="border: 1px solid black;">Frequency table</th>
			<th align="left" style="border: 1px solid black;">Answer</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td align="left" style="border: 1px solid black;">[0, 6, 4]</td>
			<td align="left" style="border: 1px solid black;">[3, 2, 3, 2, 3, 2, 3]</td>
			<td align="left" style="border: 1px solid black;">4</td>
			<td align="left" style="border: 1px solid black;">3 &rarr; 4, 2 &rarr; 3</td>
			<td align="left" style="border: 1px solid black;">3</td>
		</tr>
		<tr>
			<td align="left" style="border: 1px solid black;">[1, 5, 2]</td>
			<td align="left" style="border: 1px solid black;">[2, 3, 2, 3, 2]</td>
			<td align="left" style="border: 1px solid black;">2</td>
			<td align="left" style="border: 1px solid black;">2 &rarr; 3, 3 &rarr; 2</td>
			<td align="left" style="border: 1px solid black;">2</td>
		</tr>
		<tr>
			<td align="left" style="border: 1px solid black;">[2, 4, 1]</td>
			<td align="left" style="border: 1px solid black;">[3, 2, 3]</td>
			<td align="left" style="border: 1px solid black;">1</td>
			<td align="left" style="border: 1px solid black;">3 &rarr; 2, 2 &rarr; 1</td>
			<td align="left" style="border: 1px solid black;">3</td>
		</tr>
		<tr>
			<td align="left" style="border: 1px solid black;">[3, 3, 1]</td>
			<td align="left" style="border: 1px solid black;">[2]</td>
			<td align="left" style="border: 1px solid black;">1</td>
			<td align="left" style="border: 1px solid black;">2 &rarr; 1</td>
			<td align="left" style="border: 1px solid black;">2</td>
		</tr>
	</tbody>
</table>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li data-end="51" data-start="19"><code data-end="49" data-start="19">1 &lt;= nums.length == n &lt;= 10<sup>4</sup></code></li>
	<li data-end="82" data-start="54"><code data-end="80" data-start="54">1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li data-end="120" data-start="85"><code data-end="118" data-start="85">1 &lt;= queries.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li data-end="195" data-start="123"><code data-end="193" data-is-only-node="" data-start="155">queries[i] = [l<sub>i</sub>, r<sub>i</sub>, threshold<sub>i</sub>]</code></li>
	<li data-end="221" data-start="198"><code data-end="219" data-start="198">0 &lt;= l<sub>i</sub> &lt;= r<sub>i</sub> &lt; n</code></li>
	<li data-end="259" data-is-last-node="" data-start="224"><code data-end="259" data-is-last-node="" data-start="224">1 &lt;= threshold<sub>i</sub> &lt;= r<sub>i</sub> - l<sub>i</sub> + 1</code></li>
</ul>


## Hints

1. Use sqrt decomposition: let <code>B = int(sqrt(n))</code> and sort queries by <code>(l//B, r)</code>
2. Maintain window <code>[L,R]</code> with a frequency map <code>cnt</code> and buckets <code>bucket[f]</code> of values at count <code>f</code>
3. Slide <code>L</code> and <code>R</code> per query, updating <code>cnt</code> and <code>bucket</code>, then scan from <code>threshold</code> to max freq to find the smallest valid value or -1

## Solution

```rust
use std::collections::{HashMap, BTreeSet};

impl Solution {
    pub fn subarray_majority(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = nums.len();
        let black_block = (black_n as f64).sqrt() as usize + 1;
        let mut black_qs: Vec<_> = queries.iter().enumerate().collect();
        black_qs.sort_by_key(|&(i, q)| (q[0] as usize / black_block, q[1]));

        let mut black_f = HashMap::new();
        let mut black_fn: HashMap<i32, BTreeSet<i32>> = HashMap::new();
        let mut black_mf = 0;
        let mut black_ans = vec![-1; queries.len()];
        let (mut black_cur_l, mut black_cur_r) = (0, 0);
        Self::black_add(nums[0], &mut black_f, &mut black_fn, &mut black_mf);

        for (black_idx, black_q) in black_qs {
            let (ql, qr, qt) = (black_q[0] as usize, black_q[1] as usize, black_q[2]);
            
            while black_cur_r < qr {
                black_cur_r += 1;
                Self::black_add(nums[black_cur_r], &mut black_f, &mut black_fn, &mut black_mf);
            }
            while black_cur_l > ql {
                black_cur_l -= 1;
                Self::black_add(nums[black_cur_l], &mut black_f, &mut black_fn, &mut black_mf);
            }
            while black_cur_r > qr {
                Self::black_rem(nums[black_cur_r], &mut black_f, &mut black_fn, &mut black_mf);
                black_cur_r -= 1;
            }
            while black_cur_l < ql {
                Self::black_rem(nums[black_cur_l], &mut black_f, &mut black_fn, &mut black_mf);
                black_cur_l += 1;
            }

            let mut res = -1;
            for f in (qt..=black_mf).rev() {
                if let Some(s) = black_fn.get(&f) {
                    if let Some(&v) = s.iter().next() {
                        res = v;
                        break;
                    }
                }
            }
            black_ans[black_idx] = res;
        }
        black_ans
    }

    fn black_add(x: i32, f: &mut HashMap<i32, i32>, fn_map: &mut HashMap<i32, BTreeSet<i32>>, mf: &mut i32) {
        let old_f = f.get(&x).cloned().unwrap_or(0);
        if old_f > 0 { fn_map.get_mut(&old_f).unwrap().remove(&x); }
        let new_f = old_f + 1;
        f.insert(x, new_f);
        fn_map.entry(new_f).or_default().insert(x);
        if new_f > *mf { *mf = new_f; }
    }

    fn black_rem(x: i32, f: &mut HashMap<i32, i32>, fn_map: &mut HashMap<i32, BTreeSet<i32>>, mf: &mut i32) {
        let old_f = *f.get(&x).unwrap();
        fn_map.get_mut(&old_f).unwrap().remove(&x);
        let new_f = old_f - 1;
        if new_f > 0 {
            f.insert(x, new_f);
            fn_map.entry(new_f).or_default().insert(x);
        } else { f.remove(&x); }
        if old_f == *mf && fn_map.get(&old_f).map_or(true, |s| s.is_empty()) { *mf -= 1; }
    }
}
```