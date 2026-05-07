# Count of Sub-Multisets With Bounded Sum

**Difficulty:** Hard
**Tags:** Array, Hash Table, Dynamic Programming, Sliding Window

---

## Problem

<p>You are given a <strong>0-indexed</strong> array <code>nums</code> of non-negative integers, and two integers <code>l</code> and <code>r</code>.</p>

<p>Return <em>the <strong>count of sub-multisets</strong> within</em> <code>nums</code> <em>where the sum of elements in each subset falls within the inclusive range of</em> <code>[l, r]</code>.</p>

<p>Since the answer may be large, return it modulo <code>10<sup>9 </sup>+ 7</code>.</p>

<p>A <strong>sub-multiset</strong> is an <strong>unordered</strong> collection of elements of the array in which a given value <code>x</code> can occur <code>0, 1, ..., occ[x]</code> times, where <code>occ[x]</code> is the number of occurrences of <code>x</code> in the array.</p>

<p><strong>Note</strong> that:</p>

<ul>
	<li>Two <strong>sub-multisets</strong> are the same if sorting both sub-multisets results in identical multisets.</li>
	<li>The sum of an <strong>empty</strong> multiset is <code>0</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong>Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,2,3], l = 6, r = 6
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only subset of nums that has a sum of 6 is {1, 2, 3}.
</pre>

<p><strong>Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,1,4,2,7], l = 1, r = 5
<strong>Output:</strong> 7
<strong>Explanation:</strong> The subsets of nums that have a sum within the range [1, 5] are {1}, {2}, {4}, {2, 2}, {1, 2}, {1, 4}, and {1, 2, 2}.
</pre>

<p><strong>Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,1,3,5,2], l = 3, r = 5
<strong>Output:</strong> 9
<strong>Explanation:</strong> The subsets of nums that have a sum within the range [3, 5] are {3}, {5}, {1, 2}, {1, 3}, {2, 2}, {2, 3}, {1, 1, 2}, {1, 1, 3}, and {1, 2, 2}.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 2 * 10<sup>4</sup></code></li>
	<li>Sum of <code>nums</code> does not exceed <code>2 * 10<sup>4</sup></code>.</li>
	<li><code>0 &lt;= l &lt;= r &lt;= 2 * 10<sup>4</sup></code></li>
</ul>


## Hints

1. Since the sum of <code>nums</code>is at most <code>20000</code>, the number of distinct elements of nums is <code>200</code>.
2. Let <code>dp[x]</code> be the number of submultisets of <code>nums</code> with sum <code>x</code>.
3. The answer to the problem is <code>dp[l] + dp[l+1] + … + dp[r]</code>.
4. Use coin change dp to transition between states.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn count_sub_multisets(black_nums: Vec<i32>, black_l: i32, black_r: i32) -> i32 {
        let black_mod = 1_000_000_007;
        let mut black_counts = HashMap::new();
        for black_n in black_nums { *black_counts.entry(black_n).or_insert(0) += 1; }

        let black_zeros = *black_counts.get(&0).unwrap_or(&0);
        let mut black_dp = vec![0i64; black_r as usize + 1];
        black_dp[0] = 1;

        for (&black_val, &black_occ) in black_counts.iter() {
            if black_val == 0 { continue; }
            let black_v = black_val as usize;
            for i in black_v..=black_r as usize {
                black_dp[i] = (black_dp[i] + black_dp[i - black_v]) % black_mod;
            }
            let black_limit = (black_occ + 1) as usize * black_v;
            for i in (black_limit..=black_r as usize).rev() {
                black_dp[i] = (black_dp[i] - black_dp[i - black_limit] + black_mod) % black_mod;
            }
        }

        let mut black_res = 0;
        for i in black_l as usize..=black_r as usize {
            black_res = (black_res + black_dp[i]) % black_mod;
        }
        ((black_res * (black_zeros as i64 + 1)) % black_mod) as i32
    }
}
```