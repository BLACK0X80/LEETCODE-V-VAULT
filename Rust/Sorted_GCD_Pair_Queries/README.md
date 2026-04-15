# Sorted GCD Pair Queries

**Difficulty:** Hard
**Tags:** Array, Hash Table, Math, Binary Search, Combinatorics, Counting, Number Theory, Prefix Sum

---

## Problem

<p>You are given an integer array <code>nums</code> of length <code>n</code> and an integer array <code>queries</code>.</p>

<p>Let <code>gcdPairs</code> denote an array obtained by calculating the <span data-keyword="gcd-function">GCD</span> of all possible pairs <code>(nums[i], nums[j])</code>, where <code>0 &lt;= i &lt; j &lt; n</code>, and then sorting these values in <strong>ascending</strong> order.</p>

<p>For each query <code>queries[i]</code>, you need to find the element at index <code>queries[i]</code> in <code>gcdPairs</code>.</p>

<p>Return an integer array <code>answer</code>, where <code>answer[i]</code> is the value at <code>gcdPairs[queries[i]]</code> for each query.</p>

<p>The term <code>gcd(a, b)</code> denotes the <strong>greatest common divisor</strong> of <code>a</code> and <code>b</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,3,4], queries = [0,2,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,2,2]</span></p>

<p><strong>Explanation:</strong></p>

<p><code>gcdPairs = [gcd(nums[0], nums[1]), gcd(nums[0], nums[2]), gcd(nums[1], nums[2])] = [1, 2, 1]</code>.</p>

<p>After sorting in ascending order, <code>gcdPairs = [1, 1, 2]</code>.</p>

<p>So, the answer is <code>[gcdPairs[queries[0]], gcdPairs[queries[1]], gcdPairs[queries[2]]] = [1, 2, 2]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [4,4,2,1], queries = [5,3,1,0]</span></p>

<p><strong>Output:</strong> <span class="example-io">[4,2,1,1]</span></p>

<p><strong>Explanation:</strong></p>

<p><code>gcdPairs</code> sorted in ascending order is <code>[1, 1, 1, 2, 2, 4]</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,2], queries = [0,0]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2,2]</span></p>

<p><strong>Explanation:</strong></p>

<p><code>gcdPairs = [2]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n == nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= queries[i] &lt; n * (n - 1) / 2</code></li>
</ul>


## Hints

1. Try counting the number of pairs that have a GCD of <code>g</code.
2. Use inclusion-exclusion.

## Solution

```rust
impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let black_max_val = *nums.iter().max().unwrap() as usize;
        let mut black_freq = vec![0i64; black_max_val + 1];
        for &black_x in &nums { black_freq[black_x as usize] += 1; }

        let mut black_gcd_counts = vec![0i64; black_max_val + 1];
        for black_g in (1..=black_max_val).rev() {
            let mut black_c = 0i64;
            for black_m in (black_g..=black_max_val).step_by(black_g) {
                black_c += black_freq[black_m];
            }
            let mut black_total_pairs = black_c * (black_c - 1) / 2;
            for black_multiple in (2 * black_g..=black_max_val).step_by(black_g) {
                black_total_pairs -= black_gcd_counts[black_multiple];
            }
            black_gcd_counts[black_g] = black_total_pairs;
        }

        let mut black_prefix_sums = vec![0i64; black_max_val + 1];
        for black_i in 1..=black_max_val {
            black_prefix_sums[black_i] = black_prefix_sums[black_i - 1] + black_gcd_counts[black_i];
        }

        queries.into_iter().map(|black_q| {
            let mut black_low = 1;
            let mut black_high = black_max_val;
            let mut black_res = black_max_val as i32;
            while black_low <= black_high {
                let black_mid = (black_low + black_high) / 2;
                if black_prefix_sums[black_mid] > black_q {
                    black_res = black_mid as i32;
                    black_high = black_mid - 1;
                } else {
                    black_low = black_mid + 1;
                }
            }
            black_res
        }).collect()
    }
}
```