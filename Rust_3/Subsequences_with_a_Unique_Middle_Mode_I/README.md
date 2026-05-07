# Subsequences with a Unique Middle Mode I

**Difficulty:** Hard
**Tags:** Array, Hash Table, Math, Combinatorics

---

## Problem

<p>Given an integer array <code>nums</code>, find the number of <span data-keyword="subsequence-array">subsequences</span> of size 5 of&nbsp;<code>nums</code> with a <strong>unique middle mode</strong>.</p>

<p>Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>A <strong>mode</strong> of a sequence of numbers is defined as the element that appears the <strong>maximum</strong> number of times in the sequence.</p>

<p>A sequence of numbers contains a<strong> unique mode</strong> if it has only one mode.</p>

<p>A sequence of numbers <code>seq</code> of size 5 contains a <strong>unique middle mode</strong> if the <em>middle element</em> (<code>seq[2]</code>) is a <strong>unique mode</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1,1,1,1,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p><code>[1, 1, 1, 1, 1]</code> is the only subsequence of size 5 that can be formed, and it has a unique middle mode of 1. This subsequence can be formed in 6 different ways, so the output is 6.&nbsp;</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,2,3,3,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p><code>[1, 2, 2, 3, 4]</code> and <code>[1, 2, 3, 3, 4]</code>&nbsp;each have a unique middle mode because the number at index 2 has the greatest frequency in the subsequence. <code>[1, 2, 2, 3, 3]</code> does not have a unique middle mode because 2 and 3 appear twice.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [0,1,2,3,4,5,6,7,8]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>There is no subsequence of length 5 with a unique middle mode.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>5 &lt;= nums.length &lt;= 1000</code></li>
	<li><code><font face="monospace">-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></font></code></li>
</ul>


## Hints

1. For each index, find the number of subsequences for which it is the unique middle mode. What combinations of values can the two numbers on the left and the right take?
2. For example, we can have exactly 1 element on the left equal to the middle and all other elements differ. What other combinations are acceptable?

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn subsequences_with_middle_mode(black_a: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_n = black_a.len();
        let mut black_ans = 0i64;
        let mut black_pre = HashMap::new();
        let mut black_suf = HashMap::new();

        for &black_v in &black_a {
            *black_suf.entry(black_v).or_insert(0i64) += 1;
        }

        fn black_comb2(black_n: i64) -> i64 {
            if black_n < 2 { 0 } else { black_n * (black_n - 1) / 2 }
        }

        fn black_comb(black_n: i64, black_r: i64) -> i64 {
            if black_r == 2 { black_comb2(black_n) }
            else if black_r == 0 { 1 }
            else { 0 }
        }

        for (black_i, &black_v) in black_a.iter().enumerate() {
            let black_i_idx = black_i as i64;
            let black_n_idx = black_n as i64;
            
            if let Some(black_count) = black_suf.get_mut(&black_v) {
                *black_count -= 1;
            }

            let black_left = black_i_idx;
            let black_right = black_n_idx - 1 - black_i_idx;
            let black_pre_v = *black_pre.get(&black_v).unwrap_or(&0i64);
            let black_suf_v = *black_suf.get(&black_v).unwrap_or(&0i64);

            black_ans = (black_ans + black_comb(black_left, 2) * black_comb(black_right, 2)) % black_mod;
            black_ans = (black_ans - black_comb(black_left - black_pre_v, 2) * black_comb(black_right - black_suf_v, 2)) % black_mod;

            let mut black_keys: std::collections::HashSet<i32> = black_pre.keys().cloned().collect();
            black_keys.extend(black_suf.keys().cloned());

            for &black_x in &black_keys {
                if black_x == black_v {
                    continue;
                }

                let black_pre_x = *black_pre.get(&black_x).unwrap_or(&0i64);
                let black_suf_x = *black_suf.get(&black_x).unwrap_or(&0i64);

                let black_term1 = black_comb(black_pre_x, 2) * black_suf_v * (black_right - black_suf_v - black_suf_x);
                let black_term2 = black_comb(black_suf_x, 2) * black_pre_v * (black_left - black_pre_v - black_pre_x);
                let black_term3 = black_pre_v * black_pre_x * black_suf_x * (black_right - black_suf_v - black_suf_x);
                let black_term4 = black_suf_v * black_suf_x * black_pre_x * (black_left - black_pre_v - black_pre_x);
                
                let black_term5 = black_comb(black_pre_x, 2) * black_suf_v * black_suf_x;
                let black_term6 = black_comb(black_suf_x, 2) * black_pre_v * black_pre_x;

                black_ans = (black_ans - black_term1 - black_term2 - black_term3 - black_term4 - black_term5 - black_term6) % black_mod;
            }

            *black_pre.entry(black_v).or_insert(0i64) += 1;
        }

        ((black_ans % black_mod) + black_mod) as i32 % black_mod as i32
    }
}
```