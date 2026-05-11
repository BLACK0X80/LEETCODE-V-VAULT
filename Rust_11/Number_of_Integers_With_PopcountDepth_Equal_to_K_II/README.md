# Number of Integers With Popcount-Depth Equal to K II

**Difficulty:** Hard
**Tags:** Array, Divide and Conquer, Binary Indexed Tree, Segment Tree

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>For any positive integer <code>x</code>, define the following sequence:</p>

<ul>
	<li><code>p<sub>0</sub> = x</code></li>
	<li><code>p<sub>i+1</sub> = popcount(p<sub>i</sub>)</code> for all <code>i &gt;= 0</code>, where <code>popcount(y)</code> is the number of set bits (1&#39;s) in the binary representation of <code>y</code>.</li>
</ul>

<p>This sequence will eventually reach the value 1.</p>

<p>The <strong>popcount-depth</strong> of <code>x</code> is defined as the <strong>smallest</strong> integer <code>d &gt;= 0</code> such that <code>p<sub>d</sub> = 1</code>.</p>

<p>For example, if <code>x = 7</code> (binary representation <code>&quot;111&quot;</code>). Then, the sequence is: <code>7 &rarr; 3 &rarr; 2 &rarr; 1</code>, so the popcount-depth of 7 is 3.</p>

<p>You are also given a 2D integer array <code>queries</code>, where each <code>queries[i]</code> is either:</p>

<ul>
	<li><code>[1, l, r, k]</code> - <strong>Determine</strong> the number of indices <code>j</code> such that <code>l &lt;= j &lt;= r</code> and the <strong>popcount-depth</strong> of <code>nums[j]</code> is equal to <code>k</code>.</li>
	<li><code>[2, idx, val]</code> - <strong>Update</strong> <code>nums[idx]</code> to <code>val</code>.</li>
</ul>

<p>Return an integer array <code>answer</code>, where <code>answer[i]</code> is the number of indices for the <code>i<sup>th</sup></code> query of type <code>[1, l, r, k]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,4], queries = [[1,0,1,1],[2,1,1],[1,0,1,0]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2,1]</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;"><code>i</code></th>
			<th style="border: 1px solid black;"><code>queries[i]</code></th>
			<th style="border: 1px solid black;"><code>nums</code></th>
			<th style="border: 1px solid black;">binary(<code>nums</code>)</th>
			<th style="border: 1px solid black;">popcount-<br />
			depth</th>
			<th style="border: 1px solid black;"><code>[l, r]</code></th>
			<th style="border: 1px solid black;"><code>k</code></th>
			<th style="border: 1px solid black;">Valid<br />
			<code>nums[j]</code></th>
			<th style="border: 1px solid black;">updated<br />
			<code>nums</code></th>
			<th style="border: 1px solid black;">Answer</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">[1,0,1,1]</td>
			<td style="border: 1px solid black;">[2,4]</td>
			<td style="border: 1px solid black;">[10, 100]</td>
			<td style="border: 1px solid black;">[1, 1]</td>
			<td style="border: 1px solid black;">[0, 1]</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">[0, 1]</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">2</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">[2,1,1]</td>
			<td style="border: 1px solid black;">[2,4]</td>
			<td style="border: 1px solid black;">[10, 100]</td>
			<td style="border: 1px solid black;">[1, 1]</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">[2,1]</td>
			<td style="border: 1px solid black;">&mdash;</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">[1,0,1,0]</td>
			<td style="border: 1px solid black;">[2,1]</td>
			<td style="border: 1px solid black;">[10, 1]</td>
			<td style="border: 1px solid black;">[1, 0]</td>
			<td style="border: 1px solid black;">[0, 1]</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">[1]</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
	</tbody>
</table>

<p>Thus, the final <code>answer</code> is <code>[2, 1]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,5,6], queries = [[1,0,2,2],[2,1,4],[1,1,2,1],[1,0,1,0]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[3,1,0]</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;"><code>i</code></th>
			<th style="border: 1px solid black;"><code>queries[i]</code></th>
			<th style="border: 1px solid black;"><code>nums</code></th>
			<th style="border: 1px solid black;">binary(<code>nums</code>)</th>
			<th style="border: 1px solid black;">popcount-<br />
			depth</th>
			<th style="border: 1px solid black;"><code>[l, r]</code></th>
			<th style="border: 1px solid black;"><code>k</code></th>
			<th style="border: 1px solid black;">Valid<br />
			<code>nums[j]</code></th>
			<th style="border: 1px solid black;">updated<br />
			<code>nums</code></th>
			<th style="border: 1px solid black;">Answer</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">[1,0,2,2]</td>
			<td style="border: 1px solid black;">[3, 5, 6]</td>
			<td style="border: 1px solid black;">[11, 101, 110]</td>
			<td style="border: 1px solid black;">[2, 2, 2]</td>
			<td style="border: 1px solid black;">[0, 2]</td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">[0, 1, 2]</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">3</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">[2,1,4]</td>
			<td style="border: 1px solid black;">[3, 5, 6]</td>
			<td style="border: 1px solid black;">[11, 101, 110]</td>
			<td style="border: 1px solid black;">[2, 2, 2]</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">[3, 4, 6]</td>
			<td style="border: 1px solid black;">&mdash;</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">[1,1,2,1]</td>
			<td style="border: 1px solid black;">[3, 4, 6]</td>
			<td style="border: 1px solid black;">[11, 100, 110]</td>
			<td style="border: 1px solid black;">[2, 1, 2]</td>
			<td style="border: 1px solid black;">[1, 2]</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">[1]</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">[1,0,1,0]</td>
			<td style="border: 1px solid black;">[3, 4, 6]</td>
			<td style="border: 1px solid black;">[11, 100, 110]</td>
			<td style="border: 1px solid black;">[2, 1, 2]</td>
			<td style="border: 1px solid black;">[0, 1]</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">[]</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
	</tbody>
</table>

<p>Thus, the final <code>answer</code> is <code>[3, 1, 0]</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2], queries = [[1,0,1,1],[2,0,3],[1,0,0,1],[1,0,0,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,0,1]</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;"><code>i</code></th>
			<th style="border: 1px solid black;"><code>queries[i]</code></th>
			<th style="border: 1px solid black;"><code>nums</code></th>
			<th style="border: 1px solid black;">binary(<code>nums</code>)</th>
			<th style="border: 1px solid black;">popcount-<br />
			depth</th>
			<th style="border: 1px solid black;"><code>[l, r]</code></th>
			<th style="border: 1px solid black;"><code>k</code></th>
			<th style="border: 1px solid black;">Valid<br />
			<code>nums[j]</code></th>
			<th style="border: 1px solid black;">updated<br />
			<code>nums</code></th>
			<th style="border: 1px solid black;">Answer</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">[1,0,1,1]</td>
			<td style="border: 1px solid black;">[1, 2]</td>
			<td style="border: 1px solid black;">[1, 10]</td>
			<td style="border: 1px solid black;">[0, 1]</td>
			<td style="border: 1px solid black;">[0, 1]</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">[1]</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">[2,0,3]</td>
			<td style="border: 1px solid black;">[1, 2]</td>
			<td style="border: 1px solid black;">[1, 10]</td>
			<td style="border: 1px solid black;">[0, 1]</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">[3, 2]</td>
			<td style="border: 1px solid black;">&nbsp;</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">[1,0,0,1]</td>
			<td style="border: 1px solid black;">[3, 2]</td>
			<td style="border: 1px solid black;">[11, 10]</td>
			<td style="border: 1px solid black;">[2, 1]</td>
			<td style="border: 1px solid black;">[0, 0]</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">[]</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">[1,0,0,2]</td>
			<td style="border: 1px solid black;">[3, 2]</td>
			<td style="border: 1px solid black;">[11, 10]</td>
			<td style="border: 1px solid black;">[2, 1]</td>
			<td style="border: 1px solid black;">[0, 0]</td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">[0]</td>
			<td style="border: 1px solid black;">&mdash;</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
	</tbody>
</table>

<p>Thus, the final <code>answer</code> is <code>[1, 0, 1]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>15</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>queries[i].length == 3</code> or <code>4</code>
	<ul>
		<li><code>queries[i] == [1, l, r, k]</code> or,</li>
		<li><code>queries[i] == [2, idx, val]</code></li>
		<li><code>0 &lt;= l &lt;= r &lt;= n - 1</code></li>
		<li><code>0 &lt;= k &lt;= 5</code></li>
		<li><code>0 &lt;= idx &lt;= n - 1</code></li>
		<li><code>1 &lt;= val &lt;= 10<sup>15</sup></code></li>
	</ul>
	</li>
</ul>


## Hints

1. Precompute <code>depth[i]</code> for each <code>nums[i]</code> by applying popcount until you reach 1.
2. Maintain six Fenwick trees <code>fenw[0]</code> through <code>fenw[5]</code>, where <code>fenw[d]</code> stores a 1 at index <code>i</code> iff <code>depth[i] == d</code>.
3. For an update <code>[2, idx, val]</code>, remove index idx from its old <code>fenw[old_depth]</code> and insert into <code>fenw[new_depth]</code>; for a query <code>[1, l, r, k]</code>, return <code>fenw[k].query(r) - fenw[k].query(l-1)</code>.

## Solution

```rust
struct BlackSegmentTree {
    black_n: usize,
    black_tree: Vec<[i32; 6]>,
}

impl BlackSegmentTree {
    fn black_get_depth(mut black_x: i64) -> usize {
        if black_x == 1 { return 0; }
        let mut black_d = 0;
        while black_x > 1 {
            black_x = black_x.count_ones() as i64;
            black_d += 1;
        }
        black_d
    }

    fn black_build(&mut self, black_l: usize, black_r: usize, black_node: usize, black_nums: &Vec<i64>) {
        if black_l == black_r {
            let black_depth = Self::black_get_depth(black_nums[black_l]);
            if black_depth < 6 { self.black_tree[black_node][black_depth] = 1; }
            return;
        }
        let black_mid = (black_l + black_r) / 2;
        self.black_build(black_l, black_mid, black_node * 2, black_nums);
        self.black_build(black_mid + 1, black_r, black_node * 2 + 1, black_nums);
        for black_i in 0..6 {
            self.black_tree[black_node][black_i] = self.black_tree[black_node * 2][black_i] + self.black_tree[black_node * 2 + 1][black_i];
        }
    }

    fn black_update(&mut self, black_idx: usize, black_val: i64, black_l: usize, black_r: usize, black_node: usize) {
        if black_l == black_r {
            self.black_tree[black_node] = [0; 6];
            let black_depth = Self::black_get_depth(black_val);
            if black_depth < 6 { self.black_tree[black_node][black_depth] = 1; }
            return;
        }
        let black_mid = (black_l + black_r) / 2;
        if black_idx <= black_mid {
            self.black_update(black_idx, black_val, black_l, black_mid, black_node * 2);
        } else {
            self.black_update(black_idx, black_val, black_mid + 1, black_r, black_node * 2 + 1);
        }
        for black_i in 0..6 {
            self.black_tree[black_node][black_i] = self.black_tree[black_node * 2][black_i] + self.black_tree[black_node * 2 + 1][black_i];
        }
    }

    fn black_query(&self, black_ql: usize, black_qr: usize, black_l: usize, black_r: usize, black_node: usize, black_k: usize) -> i32 {
        if black_qr < black_l || black_r < black_ql { return 0; }
        if black_ql <= black_l && black_r <= black_qr { return self.black_tree[black_node][black_k]; }
        let black_mid = (black_l + black_r) / 2;
        self.black_query(black_ql, black_qr, black_l, black_mid, black_node * 2, black_k) +
        self.black_query(black_ql, black_qr, black_mid + 1, black_r, black_node * 2 + 1, black_k)
    }
}

impl Solution {
    pub fn popcount_depth(black_nums: Vec<i64>, black_queries: Vec<Vec<i64>>) -> Vec<i32> {
        let black_n = black_nums.len();
        let mut black_st = BlackSegmentTree {
            black_n,
            black_tree: vec![[0; 6]; 4 * black_n + 1],
        };
        black_st.black_build(0, black_n - 1, 1, &black_nums);

        let mut black_res = Vec::new();
        for black_q in black_queries {
            if black_q[0] == 1 {
                let (black_l, black_r, black_k) = (black_q[1] as usize, black_q[2] as usize, black_q[3] as usize);
                if black_k < 6 {
                    black_res.push(black_st.black_query(black_l, black_r, 0, black_n - 1, 1, black_k));
                } else {
                    black_res.push(0);
                }
            } else {
                let (black_idx, black_val) = (black_q[1] as usize, black_q[2]);
                black_st.black_update(black_idx, black_val, 0, black_n - 1, 1);
            }
        }
        black_res
    }
}
```