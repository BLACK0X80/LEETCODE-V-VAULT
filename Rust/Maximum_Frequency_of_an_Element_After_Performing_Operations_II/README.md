# Maximum Frequency of an Element After Performing Operations II

**Difficulty:** Hard
**Tags:** Array, Binary Search, Sliding Window, Sorting, Prefix Sum

---

## Problem

<p>You are given an integer array <code>nums</code> and two integers <code>k</code> and <code>numOperations</code>.</p>

<p>You must perform an <strong>operation</strong> <code>numOperations</code> times on <code>nums</code>, where in each operation you:</p>

<ul>
	<li>Select an index <code>i</code> that was <strong>not</strong> selected in any previous operations.</li>
	<li>Add an integer in the range <code>[-k, k]</code> to <code>nums[i]</code>.</li>
</ul>

<p>Return the <strong>maximum</strong> possible <span data-keyword="frequency-array">frequency</span> of any element in <code>nums</code> after performing the <strong>operations</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,4,5], k = 1, numOperations = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>We can achieve a maximum frequency of two by:</p>

<ul>
	<li>Adding 0 to <code>nums[1]</code>, after which <code>nums</code> becomes <code>[1, 4, 5]</code>.</li>
	<li>Adding -1 to <code>nums[2]</code>, after which <code>nums</code> becomes <code>[1, 4, 4]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [5,11,20,20], k = 5, numOperations = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>We can achieve a maximum frequency of two by:</p>

<ul>
	<li>Adding 0 to <code>nums[1]</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= k &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= numOperations &lt;= nums.length</code></li>
</ul>


## Hints

1. The optimal values to check are <code>nums[i] - k</code>, <code>nums[i]</code>, and <code>nums[i] + k</code>.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn max_frequency(black_nums: Vec<i32>, black_k: i32, black_num_ops: i32) -> i32 {
        let mut black_nums = black_nums;
        black_nums.sort_unstable();
        
        let black_k = black_k as i64;
        let black_num_ops = black_num_ops as i32;
        let mut black_map = HashMap::new();
        let mut black_points = std::collections::BTreeSet::new();

        for &black_x in &black_nums {
            *black_map.entry(black_x).or_insert(0) += 1;
            black_points.insert(black_x as i64);
            black_points.insert(black_x as i64 - black_k);
            black_points.insert(black_x as i64 + black_k);
        }

        let mut black_diff = std::collections::BTreeMap::new();
        for &black_x in &black_nums {
            let black_start = black_x as i64 - black_k;
            let black_end = black_x as i64 + black_k;
            *black_diff.entry(black_start).or_insert(0) += 1;
            *black_diff.entry(black_end + 1).or_insert(0) -= 1;
        }

        let mut black_ans = 0;
        let mut black_curr_coverage = 0;
        let black_diff_vec: Vec<_> = black_diff.into_iter().collect();
        let mut black_d_idx = 0;

        for black_p in black_points {
            while black_d_idx < black_diff_vec.len() && black_diff_vec[black_d_idx].0 <= black_p {
                black_curr_coverage += black_diff_vec[black_d_idx].1;
                black_d_idx += 1;
            }

            let black_count = *black_map.get(&(black_p as i32)).unwrap_or(&0);
            let black_others = black_curr_coverage - black_count;
            let black_res = black_count + black_others.min(black_num_ops);
            
            if black_res > black_ans {
                black_ans = black_res;
            }
        }

        black_ans
    }
}
```