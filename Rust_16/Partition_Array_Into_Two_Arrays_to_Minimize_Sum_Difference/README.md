# Partition Array Into Two Arrays to Minimize Sum Difference

**Difficulty:** Hard
**Tags:** Array, Two Pointers, Binary Search, Dynamic Programming, Bit Manipulation, Sorting, Ordered Set, Bitmask

---

## Problem

<p>You are given an integer array <code>nums</code> of <code>2 * n</code> integers. You need to partition <code>nums</code> into <strong>two</strong> arrays of length <code>n</code> to <strong>minimize the absolute difference</strong> of the <strong>sums</strong> of the arrays. To partition <code>nums</code>, put each element of <code>nums</code> into <strong>one</strong> of the two arrays.</p>

<p>Return <em>the <strong>minimum</strong> possible absolute difference</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="example-1" src="https://assets.leetcode.com/uploads/2021/10/02/ex1.png" style="width: 240px; height: 106px;" />
<pre>
<strong>Input:</strong> nums = [3,9,7,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> One optimal partition is: [3,9] and [7,3].
The absolute difference between the sums of the arrays is abs((3 + 9) - (7 + 3)) = 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [-36,36]
<strong>Output:</strong> 72
<strong>Explanation:</strong> One optimal partition is: [-36] and [36].
The absolute difference between the sums of the arrays is abs((-36) - (36)) = 72.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="example-3" src="https://assets.leetcode.com/uploads/2021/10/02/ex3.png" style="width: 316px; height: 106px;" />
<pre>
<strong>Input:</strong> nums = [2,-1,0,4,-2,-9]
<strong>Output:</strong> 0
<strong>Explanation:</strong> One optimal partition is: [2,4,-9] and [-1,0,-2].
The absolute difference between the sums of the arrays is abs((2 + 4 + -9) - (-1 + 0 + -2)) = 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 15</code></li>
	<li><code>nums.length == 2 * n</code></li>
	<li><code>-10<sup>7</sup> &lt;= nums[i] &lt;= 10<sup>7</sup></code></li>
</ul>


## Hints

1. The target sum for the two partitions is sum(nums) / 2.
2. Could you reduce the time complexity if you arbitrarily divide nums into two halves (two arrays)? Meet-in-the-Middle?
3. For both halves, pre-calculate a 2D array where the kth index will store all possible sum values if only k elements from this half are added.
4. For each sum of k elements in the first half, find the best sum of n-k elements in the second half such that the two sums add up to a value closest to the target sum from hint 1. These two subsets will form one array of the partition.

## Solution

```rust
impl Solution {
    pub fn minimum_difference(black_nums: Vec<i32>) -> i32 {
        let black_2n = black_nums.len();
        let black_n = black_2n / 2;
        let black_sum_total: i64 = black_nums.iter().map(|&x| x as i64).sum();
        
        let black_left_half = &black_nums[0..black_n];
        let black_right_half = &black_nums[black_n..black_2n];

        let mut black_left_sums = vec![vec![]; black_n + 1];
        let mut black_right_sums = vec![vec![]; black_n + 1];

        for black_i in 0..(1 << black_n) {
            let mut black_cur_sum: i64 = 0;
            let mut black_count = 0;
            for black_bit in 0..black_n {
                if (black_i >> black_bit) & 1 == 1 {
                    black_cur_sum += black_left_half[black_bit] as i64;
                    black_count += 1;
                }
            }
            black_left_sums[black_count].push(black_cur_sum);
        }

        for black_i in 0..(1 << black_n) {
            let mut black_cur_sum: i64 = 0;
            let mut black_count = 0;
            for black_bit in 0..black_n {
                if (black_i >> black_bit) & 1 == 1 {
                    black_cur_sum += black_right_half[black_bit] as i64;
                    black_count += 1;
                }
            }
            black_right_sums[black_count].push(black_cur_sum);
        }

        for black_i in 0..=black_n {
            black_right_sums[black_i].sort_unstable();
        }

        let mut black_min_diff = i64::MAX;

        for black_k in 0..=black_n {
            let black_needed_in_right = black_n - black_k;
            let black_target = black_sum_total / 2;

            for &black_s1 in &black_left_sums[black_k] {
                let black_s_target = black_target - black_s1;
                
                let black_vec = &black_right_sums[black_needed_in_right];
                let black_pos = match black_vec.binary_search(&black_s_target) {
                    Ok(black_p) => black_p,
                    Err(black_p) => black_p,
                };

                if black_pos < black_vec.len() {
                    let black_s2 = black_vec[black_pos];
                    let black_s_full = black_s1 + black_s2;
                    black_min_diff = black_min_diff.min((black_sum_total - 2 * black_s_full).abs());
                }
                if black_pos > 0 {
                    let black_s2 = black_vec[black_pos - 1];
                    let black_s_full = black_s1 + black_s2;
                    black_min_diff = black_min_diff.min((black_sum_total - 2 * black_s_full).abs());
                }
            }
        }

        black_min_diff as i32
    }
}
```