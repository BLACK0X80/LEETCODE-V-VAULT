# Minimum Inversion Count in Subarrays of Fixed Length

**Difficulty:** Hard
**Tags:** Array, Segment Tree, Sliding Window

---

## Problem

<p>You are given an integer array <code>nums</code> of length <code>n</code> and an integer <code>k</code>.</p>

<p>An <strong>inversion</strong> is a pair of indices <code>(i, j)</code> from <code>nums</code> such that <code>i &lt; j</code> and <code>nums[i] &gt; nums[j]</code>.</p>

<p>The <strong>inversion count</strong> of a <strong><span data-keyword="subarray-nonempty">subarray</span></strong> is the number of inversions within it.</p>

<p>Return the <strong>minimum</strong> inversion count among all <strong>subarrays</strong> of <code>nums</code> with length <code>k</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,1,2,5,4], k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>We consider all subarrays of length <code>k = 3</code> (indices below are relative to each subarray):</p>

<ul>
	<li><code>[3, 1, 2]</code> has 2 inversions: <code>(0, 1)</code> and <code>(0, 2)</code>.</li>
	<li><code>[1, 2, 5]</code> has 0 inversions.</li>
	<li><code>[2, 5, 4]</code> has 1 inversion: <code>(1, 2)</code>.</li>
</ul>

<p>The minimum inversion count among all subarrays of length <code>3</code> is 0, achieved by subarray <code>[1, 2, 5]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [5,3,2,1], k = 4</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>There is only one subarray of length <code>k = 4</code>: <code>[5, 3, 2, 1]</code>.<br />
Within this subarray, the inversions are: <code>(0, 1)</code>, <code>(0, 2)</code>, <code>(0, 3)</code>, <code>(1, 2)</code>, <code>(1, 3)</code>, and <code>(2, 3)</code>.<br />
Total inversions is 6, so the minimum inversion count is 6.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,1], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>All subarrays of length <code>k = 1</code> contain only one element, so no inversions are possible.<br />
The minimum inversion count is therefore 0.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= k &lt;= n</code></li>
</ul>


## Hints

1. Compress all numbers to integers in the range <code>1</code> to <code>n</code>.
2. Use a Fenwick tree (BIT) to maintain counts of the numbers.
3. When adding an element at the back, query how many elements are larger than it; when deleting an element from the front, query how many elements are smaller and update the tree accordingly.

## Solution

```rust
impl Solution {
    pub fn min_inversion_count(black_nums: Vec<i32>, black_k: i32) -> i64 {
        let (black_n, black_k) = (black_nums.len(), black_k as usize);
        let mut black_sorted = black_nums.clone();
        black_sorted.sort_unstable(); black_sorted.dedup();
        let black_m = black_sorted.len();
        let (mut black_bit, mut black_inv, mut black_ans) = (vec![0i64; black_m + 1], 0i64, i64::MAX);
        for black_i in 0..black_n {
            let black_in = black_sorted.binary_search(&black_nums[black_i]).unwrap() + 1;
            if black_i >= black_k {
                let black_out = black_sorted.binary_search(&black_nums[black_i - black_k]).unwrap() + 1;
                let (mut black_idx, mut black_s) = (black_out - 1, 0);
                while black_idx > 0 { black_s += black_bit[black_idx]; black_idx -= (black_idx as i32 & -(black_idx as i32)) as usize; }
                black_inv -= black_s;
                let mut black_idx = black_out;
                while black_idx <= black_m { black_bit[black_idx] -= 1; black_idx += (black_idx as i32 & -(black_idx as i32)) as usize; }
            }
            let (mut black_idx, mut black_s) = (black_in, 0);
            while black_idx > 0 { black_s += black_bit[black_idx]; black_idx -= (black_idx as i32 & -(black_idx as i32)) as usize; }
            black_inv += (black_i.min(black_k - 1) as i64 - black_s);
            let mut black_idx = black_in;
            while black_idx <= black_m { black_bit[black_idx] += 1; black_idx += (black_idx as i32 & -(black_idx as i32)) as usize; }
            if black_i >= black_k - 1 { black_ans = black_ans.min(black_inv); }
        }
        black_ans
    }
}
```