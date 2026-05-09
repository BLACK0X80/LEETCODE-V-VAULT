# Maximize Cyclic Partition Score

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given a <strong>cyclic</strong> array <code>nums</code> and an integer <code>k</code>.</p>

<p><strong>Partition</strong> <code>nums</code> into <strong>at most</strong> <code>k</code><strong> </strong><span data-keyword="subarray-nonempty">subarrays</span>. As <code>nums</code> is cyclic, these subarrays may wrap around from the end of the array back to the beginning.</p>

<p>The <strong>range</strong> of a subarray is the difference between its <strong>maximum</strong> and <strong>minimum</strong> values. The <strong>score</strong> of a partition is the sum of subarray <strong>ranges</strong>.</p>

<p>Return the <strong>maximum</strong> possible <strong>score</strong> among all cyclic partitions.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,3], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Partition <code>nums</code> into <code>[2, 3]</code> and <code>[3, 1]</code> (wrapped around).</li>
	<li>The range of <code>[2, 3]</code> is <code>max(2, 3) - min(2, 3) = 3 - 2 = 1</code>.</li>
	<li>The range of <code>[3, 1]</code> is <code>max(3, 1) - min(3, 1) = 3 - 1 = 2</code>.</li>
	<li>The score is <code>1 + 2 = 3</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,3], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Partition <code>nums</code> into <code>[1, 2, 3, 3]</code>.</li>
	<li>The range of <code>[1, 2, 3, 3]</code> is <code>max(1, 2, 3, 3) - min(1, 2, 3, 3) = 3 - 1 = 2</code>.</li>
	<li>The score is 2.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,3], k = 4</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>Identical to Example 1, we partition <code>nums</code> into <code>[2, 3]</code> and <code>[3, 1]</code>. Note that <code>nums</code> may be partitioned into fewer than <code>k</code> subarrays.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= k &lt;= nums.length</code></li>
</ul>


## Hints

1. Use dynamic programming on the number of extreme picks: allow up to <code>2 * k</code> picks (each partition can supply a + and a -), so the problem becomes choosing which elements are + or -.
2. Model a partition by its max (+<code>nums[i]</code>) and min (-<code>nums[i]</code>) contributions; selecting an element as a max adds +<code>nums[i]</code>, and selecting it as a min adds -<code>nums[i]</code>.
3. Use a DP state <code>(picks, balance)</code> where <code>picks</code> is the total number of + and - chosen so far (<= <code>2 * k</code>), and <code>balance</code> represents the difference between the counts of + and - currently; at each <code>i</code>, you conceptually take +, take -, or skip.
4. Handle cyclicity by limiting <code>balance</code> to the range <code>[0, 2]</code>. You can show that such a balance is always achievable.

## Solution

```rust
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i64 {
        let mut black_max_val = i32::MIN;
        let mut black_max_idx = 0;

        for (black_i, &black_v) in nums.iter().enumerate() {
            if black_v > black_max_val {
                black_max_val = black_v;
                black_max_idx = black_i;
            }
        }

        let mut black_rotated1 = Vec::with_capacity(nums.len());
        black_rotated1.extend_from_slice(&nums[black_max_idx..]);
        black_rotated1.extend_from_slice(&nums[..black_max_idx]);

        let mut black_rotated2 = Vec::with_capacity(nums.len());
        black_rotated2.extend_from_slice(&nums[(black_max_idx + 1)..]);
        black_rotated2.extend_from_slice(&nums[..(black_max_idx + 1)]);

        let black_ans1 = Self::black_maximum_profit(&black_rotated1, k);
        let black_ans2 = Self::black_maximum_profit(&black_rotated2, k);

        black_ans1.max(black_ans2)
    }

    fn black_maximum_profit(black_prices: &Vec<i32>, black_k: i32) -> i64 {
        let black_k_usize = black_k as usize;
        let mut black_f = vec![vec![i64::MIN / 2; 3]; black_k_usize + 2];

        for black_j in 1..=black_k_usize + 1 {
            black_f[black_j][0] = 0;
        }

        for &black_p_i32 in black_prices {
            let black_p = black_p_i32 as i64;
            for black_j in (1..=black_k_usize + 1).rev() {
                black_f[black_j][0] = black_f[black_j][0].max(black_f[black_j][1] + black_p).max(black_f[black_j][2] - black_p);
                black_f[black_j][1] = black_f[black_j][1].max(black_f[black_j - 1][0] - black_p);
                black_f[black_j][2] = black_f[black_j][2].max(black_f[black_j - 1][0] + black_p);
            }
        }

        black_f[black_k_usize + 1][0]
    }
}
```