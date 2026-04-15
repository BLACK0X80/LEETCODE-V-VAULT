# Minimum Increments for Target Multiples in an Array

**Difficulty:** Hard
**Tags:** Array, Math, Dynamic Programming, Bit Manipulation, Number Theory, Bitmask

---

## Problem

<p>You are given two arrays, <code>nums</code> and <code>target</code>.</p>

<p>In a single operation, you may increment any element of <code>nums</code> by 1.</p>

<p>Return <strong>the minimum number</strong> of operations required so that each element in <code>target</code> has <strong>at least</strong> one multiple in <code>nums</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3], target = [4]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The minimum number of operations required to satisfy the condition is 1.</p>

<ul>
	<li>Increment 3 to 4 with just one operation, making 4 a multiple of itself.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [8,4], target = [10,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The minimum number of operations required to satisfy the condition is 2.</p>

<ul>
	<li>Increment 8 to 10 with 2 operations, making 10 a multiple of both 5 and 10.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [7,9,10], target = [7]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>Target 7 already has a multiple in nums, so no additional operations are needed.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= target.length &lt;= 4</code></li>
	<li><code>target.length &lt;= nums.length</code></li>
	<li><code>1 &lt;= nums[i], target[i] &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Use bitmask dynamic programming.

## Solution

```rust
impl Solution {
    pub fn minimum_increments(black_nums: Vec<i32>, black_target: Vec<i32>) -> i32 {
        let black_m = black_target.len();
        let mut black_lcm_map = vec![0i64; 1 << black_m];

        fn black_gcd(mut a: i64, mut b: i64) -> i64 {
            while b != 0 {
                a %= b;
                std::mem::swap(&mut a, &mut b);
            }
            a
        }

        fn black_lcm(a: i64, b: i64) -> i64 {
            if a == 0 || b == 0 { return 0; }
            let g = black_gcd(a, b);
            let black_res = a.saturating_mul(b / g);
            if black_res > 200_000 { 200_001 } else { black_res }
        }

        for black_mask in 1..(1 << black_m) {
            let mut black_res_lcm = 0;
            for black_i in 0..black_m {
                if (black_mask >> black_i) & 1 == 1 {
                    if black_res_lcm == 0 {
                        black_res_lcm = black_target[black_i] as i64;
                    } else {
                        black_res_lcm = black_lcm(black_res_lcm, black_target[black_i] as i64);
                    }
                }
            }
            black_lcm_map[black_mask] = black_res_lcm;
        }

        let mut black_dp = vec![i32::MAX / 2; 1 << black_m];
        black_dp[0] = 0;

        for &black_num in &black_nums {
            let mut black_next_dp = black_dp.clone();
            for black_mask in 1..(1 << black_m) {
                let black_l = black_lcm_map[black_mask];
                let black_rem = (black_num as i64) % black_l;
                let black_cost = if black_rem == 0 { 0 } else { (black_l - black_rem) as i32 };
                
                for black_prev_mask in 0..(1 << black_m) {
                    let black_new_mask = black_prev_mask | black_mask;
                    let black_total_cost = black_dp[black_prev_mask] + black_cost;
                    if black_total_cost < black_next_dp[black_new_mask] {
                        black_next_dp[black_new_mask] = black_total_cost;
                    }
                }
            }
            black_dp = black_next_dp;
        }

        black_dp[(1 << black_m) - 1]
    }
}
```