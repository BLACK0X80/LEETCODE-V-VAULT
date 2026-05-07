# Find the Sum of Subsequence Powers

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Sorting

---

## Problem

<p>You are given an integer array <code>nums</code> of length <code>n</code>, and a <strong>positive</strong> integer <code>k</code>.</p>

<p>The <strong>power</strong> of a <span data-keyword="subsequence-array">subsequence</span> is defined as the <strong>minimum</strong> absolute difference between <strong>any</strong> two elements in the subsequence.</p>

<p>Return <em>the <strong>sum</strong> of <strong>powers</strong> of <strong>all</strong> subsequences of </em><code>nums</code><em> which have length</em> <strong><em>equal to</em></strong> <code>k</code>.</p>

<p>Since the answer may be large, return it <strong>modulo</strong> <code>10<sup>9 </sup>+ 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,4], k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>There are 4 subsequences in <code>nums</code> which have length 3: <code>[1,2,3]</code>, <code>[1,3,4]</code>, <code>[1,2,4]</code>, and <code>[2,3,4]</code>. The sum of powers is <code>|2 - 3| + |3 - 4| + |2 - 1| + |3 - 4| = 4</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,2], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>The only subsequence in <code>nums</code> which has length 2 is&nbsp;<code>[2,2]</code>. The sum of powers is <code>|2 - 2| = 0</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,3,-1], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">10</span></p>

<p><strong>Explanation:</strong></p>

<p>There are 3 subsequences in <code>nums</code> which have length 2: <code>[4,3]</code>, <code>[4,-1]</code>, and <code>[3,-1]</code>. The sum of powers is <code>|4 - 3| + |4 - (-1)| + |3 - (-1)| = 10</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n == nums.length &lt;= 50</code></li>
	<li><code>-10<sup>8</sup> &lt;= nums[i] &lt;= 10<sup>8</sup> </code></li>
	<li><code>2 &lt;= k &lt;= n</code></li>
</ul>


## Hints

1. Sort <code>nums</code>.
2. There are at most <code>n<sup>2</sup></code> distinct differences.
3. For a particular difference <code>d</code>, let <code>dp[len][i][j]</code> be the number of subsequences of length <code>len</code> in the subarray <code>nums[0..i]</code> where the last element picked was at index <code>j</code>.
4. For each index, we can check if it can be picked if <code>nums[i] - nums[j] <= d</code>.

## Solution

```rust
impl Solution {
    pub fn sum_of_powers(mut black_nums: Vec<i32>, black_k: i32) -> i32 {
        black_nums.sort_unstable();
        let black_n = black_nums.len();
        let black_mod = 1_000_000_007;
        let mut black_dp = vec![vec![std::collections::HashMap::new(); black_k as usize + 1]; black_n];

        let bravexuneth = &black_nums;

        for black_i in 0..black_n {
            black_dp[black_i][1].insert(i32::MAX, 1);
            for black_j in 0..black_i {
                let black_diff = bravexuneth[black_i] - bravexuneth[black_j];
                for black_len in 2..=black_k as usize {
                    let black_prev_map = black_dp[black_j][black_len - 1].clone();
                    for (&black_min_d, &black_count) in black_prev_map.iter() {
                        let black_new_min = black_min_d.min(black_diff);
                        let black_entry = black_dp[black_i][black_len].entry(black_new_min).or_insert(0);
                        *black_entry = (*black_entry + black_count) % black_mod;
                    }
                }
            }
        }

        let mut black_res = 0i64;
        for black_i in 0..black_n {
            for (&black_d, &black_c) in black_dp[black_i][black_k as usize].iter() {
                black_res = (black_res + black_d as i64 * black_c as i64) % black_mod as i64;
            }
        }
        black_res as i32
    }
}
```