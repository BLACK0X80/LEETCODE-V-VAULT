# Peaks in Array

**Difficulty:** Hard
**Tags:** Array, Binary Indexed Tree, Segment Tree

---

## Problem

<p>A <strong>peak</strong> in an array <code>arr</code> is an element that is <strong>greater</strong> than its previous and next element in <code>arr</code>.</p>

<p>You are given an integer array <code>nums</code> and a 2D integer array <code>queries</code>.</p>

<p>You have to process queries of two types:</p>

<ul>
	<li><code>queries[i] = [1, l<sub>i</sub>, r<sub>i</sub>]</code>, determine the count of <strong>peak</strong> elements in the <span data-keyword="subarray">subarray</span> <code>nums[l<sub>i</sub>..r<sub>i</sub>]</code>.<!-- notionvc: 73b20b7c-e1ab-4dac-86d0-13761094a9ae --></li>
	<li><code>queries[i] = [2, index<sub>i</sub>, val<sub>i</sub>]</code>, change <code>nums[index<sub>i</sub>]</code> to <code><font face="monospace">val<sub>i</sub></font></code>.</li>
</ul>

<p>Return an array <code>answer</code> containing the results of the queries of the first type in order.<!-- notionvc: a9ccef22-4061-4b5a-b4cc-a2b2a0e12f30 --></p>

<p><strong>Notes:</strong></p>

<ul>
	<li>The <strong>first</strong> and the <strong>last</strong> element of an array or a subarray<!-- notionvc: fcffef72-deb5-47cb-8719-3a3790102f73 --> <strong>cannot</strong> be a peak.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,1,4,2,5], queries = [[2,3,4],[1,0,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0]</span></p>

<p><strong>Explanation:</strong></p>

<p>First query: We change <code>nums[3]</code> to 4 and <code>nums</code> becomes <code>[3,1,4,4,5]</code>.</p>

<p>Second query: The number of peaks in the <code>[3,1,4,4,5]</code> is 0.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,1,4,2,1,5], queries = [[2,2,4],[1,0,2],[1,0,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,1]</span></p>

<p><strong>Explanation:</strong></p>

<p>First query: <code>nums[2]</code> should become 4, but it is already set to 4.</p>

<p>Second query: The number of peaks in the <code>[4,1,4]</code> is 0.</p>

<p>Third query: The second 4 is a peak in the <code>[4,1,4,2,1]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>queries[i][0] == 1</code> or <code>queries[i][0] == 2</code></li>
	<li>For all <code>i</code> that:
	<ul>
		<li><code>queries[i][0] == 1</code>: <code>0 &lt;= queries[i][1] &lt;= queries[i][2] &lt;= nums.length - 1</code></li>
		<li><code>queries[i][0] == 2</code>: <code>0 &lt;= queries[i][1] &lt;= nums.length - 1</code>, <code>1 &lt;= queries[i][2] &lt;= 10<sup>5</sup></code></li>
	</ul>
	</li>
</ul>


## Hints

1. Let <code>p[i]</code> be whether <code>nums[i]</code> is a peak in the original array. Namely <code>p[i] = nums[i] > nums[i - 1] && nums[i] > nums[i + 1]</code>.
2. Updating <code>nums[i]</code>, only affects <code>p[i]</code>, <code>p[i - 1]</code> and <code>p[i + 1]</code>. We can recalculate the 3 values in constant time.
3. The answer for <code>[l<sub>i</sub>, r<sub>i</sub>]</code> is <code>p[l<sub>i</sub> + 1] + p[l<sub>i</sub> + 2] + … + p[r<sub>i</sub> - 1]</code> (note that <code>l<sub>i</sub></code> and <code>r<sub>i</sub></code> are not included).
4. Use some data structures (i.e. segment tree or binary indexed tree) to maintain the subarray sum efficiently.

## Solution

```rust
struct BlackBIT { black_tree: Vec<i32> }
impl BlackBIT {
    fn new(black_n: usize) -> Self { Self { black_tree: vec![0; black_n + 1] } }
    fn update(&mut self, mut black_i: usize, black_val: i32) {
        black_i += 1;
        while black_i < self.black_tree.len() { self.black_tree[black_i] += black_val; black_i += (black_i as i32 & -(black_i as i32)) as usize; }
    }
    fn query(&self, mut black_i: usize) -> i32 {
        let mut black_sum = 0; black_i += 1;
        while black_i > 0 { black_sum += self.black_tree[black_i]; black_i -= (black_i as i32 & -(black_i as i32)) as usize; }
        black_sum
    }
}
impl Solution {
    pub fn count_of_peaks(mut black_nums: Vec<i32>, black_queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_nums.len();
        let mut black_bit = BlackBIT::new(black_n);
        let mut black_is_peak = vec![false; black_n];
        let black_check = |black_i: usize, black_v: &[i32]| {
            if black_i <= 0 || black_i >= black_v.len() - 1 { false }
            else { black_v[black_i] > black_v[black_i-1] && black_v[black_i] > black_v[black_i+1] }
        };
        for black_i in 1..black_n-1 { if black_check(black_i, &black_nums) { black_is_peak[black_i] = true; black_bit.update(black_i, 1); } }
        let mut black_ans = vec![];
        for black_q in black_queries {
            if black_q[0] == 1 {
                let (black_l, black_r) = (black_q[1] as usize, black_q[2] as usize);
                if black_r - black_l < 2 { black_ans.push(0); }
                else { black_ans.push(black_bit.query(black_r - 1) - black_bit.query(black_l)); }
            } else {
                let (black_idx, black_val) = (black_q[1] as usize, black_q[2]);
                for black_j in black_idx.saturating_sub(1)..=(black_idx + 1).min(black_n - 1) {
                    if black_is_peak[black_j] { black_bit.update(black_j, -1); black_is_peak[black_j] = false; }
                }
                black_nums[black_idx] = black_val;
                for black_j in black_idx.saturating_sub(1)..=(black_idx + 1).min(black_n - 1) {
                    if black_check(black_j, &black_nums) { black_is_peak[black_j] = true; black_bit.update(black_j, 1); }
                }
            }
        }
        black_ans
    }
}
```