# Sum of Sortable Integers

**Difficulty:** Hard
**Tags:** Array, Math, Sorting, Enumeration

---

## Problem

<p>You are given an integer array <code>nums</code> of length <code>n</code>.</p>

<p>An integer <code>k</code> is called <strong>sortable</strong> if <code>k</code> <strong>divides</strong> <code>n</code> and you can sort <code>nums</code> in <strong>non-decreasing</strong> order by sequentially performing the following operations:</p>

<ul>
	<li>Partition <code>nums</code> into <strong>consecutive <span data-keyword="subarray-nonempty">subarrays</span></strong> of length <code>k</code>.</li>
	<li><strong>Cyclically rotate each subarray independently</strong> any number of times to the left or to the right.</li>
</ul>

<p>Return an integer denoting the sum of all possible sortable integers <code>k</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,1,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong>​​​​​​​</p>

<ul>
	<li>For <code>n = 3</code>, possible divisors are 1 and 3.</li>
	<li>For <code>k = 1</code>: each subarray has one element. No rotation can sort the array.</li>
	<li>For <code>k = 3</code>: the single subarray <code>[3, 1, 2]</code> can be rotated once to produce <code>[1, 2, 3]</code>, which is sorted.</li>
	<li>Only <code>k = 3</code> is sortable. Hence, the answer is 3.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [7,6,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>For <code>n = 3</code>, possible divisors are 1 and 3.</li>
	<li>For <code>k = 1</code>: each subarray has one element. No rotation can sort the array.</li>
	<li>For <code>k = 3</code>: the single subarray <code>[7, 6, 5]</code> cannot be rotated into non-decreasing order.</li>
	<li>No <code>k</code> is sortable. Hence, the answer is 0.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [5,8]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong>​​​​​​​</p>

<ul>
	<li>For <code>n = 2</code>, possible divisors are 1 and 2.</li>
	<li>Since <code>[5, 8]</code> is already sorted, every divisor is sortable. Hence, the answer is <code>1 + 2 = 3</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. First, observe that <code>k</code> must divide <code>n</code>, since the array is partitioned into subarrays of length <code>k</code>. Iterate over all divisors of <code>n</code>.
2. Let <code>s</code> be the sorted version of the array. For each divisor <code>k</code>, check every block <code>[i, i+k)</code>.
3. A block can be rotated to match its sorted counterpart if the sorted segment <code>s[i..i+k-1]</code> appears as a cyclic rotation of <code>nums[i..i+k-1]</code>.
4. You can test this by checking whether the sorted segment exists inside two concatenated copies of the block. If all blocks satisfy this, then <code>k</code> is sortable.

## Solution

```rust
impl Solution {
    pub fn sortable_integers(black_nums: Vec<i32>) -> i32 {
        let (black_n, mut black_ans) = (black_nums.len(), 0);
        for black_k in (1..=black_n).filter(|&black_k| black_n % black_k == 0) {
            let (mut black_ok, mut black_prev_max) = (true, 0);
            for black_c in black_nums.chunks(black_k) {
                let black_d_idx = (0..black_k).find(|&black_i| black_c[black_i] < black_c[(black_i + black_k - 1) % black_k]).unwrap_or(0);
                if (0..black_k - 1).filter(|&black_i| black_c[(black_d_idx + black_i) % black_k] > black_c[(black_d_idx + black_i + 1) % black_k]).count() > 0 
                   || black_c[black_d_idx] < black_prev_max { black_ok = false; break; }
                black_prev_max = black_c[(black_d_idx + black_k - 1) % black_k];
            }
            if black_ok { black_ans += black_k as i32; }
        }
        black_ans
    }
}
```