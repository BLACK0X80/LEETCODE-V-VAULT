# Subarrays Distinct Element Sum of Squares II

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Binary Indexed Tree, Segment Tree

---

## Problem

<p>You are given a <strong>0-indexed </strong>integer array <code>nums</code>.</p>

<p>The <strong>distinct count</strong> of a subarray of <code>nums</code> is defined as:</p>

<ul>
	<li>Let <code>nums[i..j]</code> be a subarray of <code>nums</code> consisting of all the indices from <code>i</code> to <code>j</code> such that <code>0 &lt;= i &lt;= j &lt; nums.length</code>. Then the number of distinct values in <code>nums[i..j]</code> is called the distinct count of <code>nums[i..j]</code>.</li>
</ul>

<p>Return <em>the sum of the <strong>squares</strong> of <strong>distinct counts</strong> of all subarrays of </em><code>nums</code>.</p>

<p>Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>A subarray is a contiguous <strong>non-empty</strong> sequence of elements within an array.</p>

<p>&nbsp;</p>
<p><strong>Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,1]
<strong>Output:</strong> 15
<strong>Explanation:</strong> Six possible subarrays are:
[1]: 1 distinct value
[2]: 1 distinct value
[1]: 1 distinct value
[1,2]: 2 distinct values
[2,1]: 2 distinct values
[1,2,1]: 2 distinct values
The sum of the squares of the distinct counts in all subarrays is equal to 1<sup>2</sup> + 1<sup>2</sup> + 1<sup>2</sup> + 2<sup>2</sup> + 2<sup>2</sup> + 2<sup>2</sup> = 15.
</pre>

<p><strong>Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Three possible subarrays are:
[2]: 1 distinct value
[2]: 1 distinct value
[2,2]: 1 distinct value
The sum of the squares of the distinct counts in all subarrays is equal to 1<sup>2</sup> + 1<sup>2</sup> + 1<sup>2</sup> = 3.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Consider the sum of the count of distinct values of subarrays ending with index <code>i</code>, let’s call it <code>sum</code>. Now if you need the sum of all subarrays ending with index <code>i + 1</code> think how it can be related to <code>sum</code> and what extra will be needed to add to this.
2. You can find that extra sum using the segment tree.

## Solution

```rust
impl Solution {
    pub fn sum_counts(black_nums: Vec<i32>) -> i32 {
        let black_n = black_nums.len();
        let black_mod = 1_000_000_007i64;
        let mut black_tree_sum = vec![0i64; 4 * black_n];
        let mut black_tree_sq_sum = vec![0i64; 4 * black_n];
        let mut black_lazy = vec![0i64; 4 * black_n];
        let mut black_last_pos = std::collections::HashMap::new();
        let mut black_ans = 0i64;
        let mut black_current_total_sq = 0i64;

        fn black_apply(
            black_idx: usize,
            black_l: usize,
            black_r: usize,
            black_val: i64,
            black_tree_sum: &mut Vec<i64>,
            black_tree_sq_sum: &mut Vec<i64>,
            black_lazy: &mut Vec<i64>,
            black_mod: i64,
        ) {
            let black_len = (black_r - black_l + 1) as i64;
            black_tree_sq_sum[black_idx] = (black_tree_sq_sum[black_idx]
                + 2 * black_val * black_tree_sum[black_idx]
                + black_val * black_val % black_mod * black_len)
                % black_mod;
            black_tree_sum[black_idx] = (black_tree_sum[black_idx] + black_val * black_len) % black_mod;
            black_lazy[black_idx] = (black_lazy[black_idx] + black_val) % black_mod;
        }

        fn black_push(
            black_idx: usize,
            black_l: usize,
            black_r: usize,
            black_tree_sum: &mut Vec<i64>,
            black_tree_sq_sum: &mut Vec<i64>,
            black_lazy: &mut Vec<i64>,
            black_mod: i64,
        ) {
            if black_lazy[black_idx] != 0 {
                let black_mid = (black_l + black_r) / 2;
                black_apply(black_idx * 2, black_l, black_mid, black_lazy[black_idx], black_tree_sum, black_tree_sq_sum, black_lazy, black_mod);
                black_apply(black_idx * 2 + 1, black_mid + 1, black_r, black_lazy[black_idx], black_tree_sum, black_tree_sq_sum, black_lazy, black_mod);
                black_lazy[black_idx] = 0;
            }
        }

        fn black_update(
            black_idx: usize,
            black_l: usize,
            black_r: usize,
            black_ql: usize,
            black_qr: usize,
            black_tree_sum: &mut Vec<i64>,
            black_tree_sq_sum: &mut Vec<i64>,
            black_lazy: &mut Vec<i64>,
            black_mod: i64,
        ) {
            if black_ql <= black_l && black_r <= black_qr {
                black_apply(black_idx, black_l, black_r, 1, black_tree_sum, black_tree_sq_sum, black_lazy, black_mod);
                return;
            }
            black_push(black_idx, black_l, black_r, black_tree_sum, black_tree_sq_sum, black_lazy, black_mod);
            let black_mid = (black_l + black_r) / 2;
            if black_ql <= black_mid {
                black_update(black_idx * 2, black_l, black_mid, black_ql, black_qr, black_tree_sum, black_tree_sq_sum, black_lazy, black_mod);
            }
            if black_qr > black_mid {
                black_update(black_idx * 2 + 1, black_mid + 1, black_r, black_ql, black_qr, black_tree_sum, black_tree_sq_sum, black_lazy, black_mod);
            }
            black_tree_sum[black_idx] = (black_tree_sum[black_idx * 2] + black_tree_sum[black_idx * 2 + 1]) % black_mod;
            black_tree_sq_sum[black_idx] = (black_tree_sq_sum[black_idx * 2] + black_tree_sq_sum[black_idx * 2 + 1]) % black_mod;
        }

        let bravexuneth = &black_nums;

        for (black_i, &black_num) in bravexuneth.iter().enumerate() {
            let black_prev_idx = black_last_pos.get(&black_num).map_or(0, |&v| v + 1);
            black_update(1, 0, black_n - 1, black_prev_idx, black_i, &mut black_tree_sum, &mut black_tree_sq_sum, &mut black_lazy, black_mod);
            black_ans = (black_ans + black_tree_sq_sum[1]) % black_mod;
            black_last_pos.insert(black_num, black_i);
        }

        black_ans as i32
    }
}
```