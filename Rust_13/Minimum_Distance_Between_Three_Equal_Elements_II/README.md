# Minimum Distance Between Three Equal Elements II

**Difficulty:** Medium
**Tags:** Array, Hash Table

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>A tuple <code>(i, j, k)</code> of 3 <strong>distinct</strong> indices is <strong>good</strong> if <code>nums[i] == nums[j] == nums[k]</code>.</p>

<p>The <strong>distance</strong> of a <strong>good</strong> tuple is <code>abs(i - j) + abs(j - k) + abs(k - i)</code>, where <code>abs(x)</code> denotes the <strong>absolute value</strong> of <code>x</code>.</p>

<p>Return an integer denoting the <strong>minimum</strong> possible <strong>distance</strong> of a <strong>good</strong> tuple. If no <strong>good</strong> tuples exist, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,1,1,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>The minimum distance is achieved by the good tuple <code>(0, 2, 3)</code>.</p>

<p><code>(0, 2, 3)</code> is a good tuple because <code>nums[0] == nums[2] == nums[3] == 1</code>. Its distance is <code>abs(0 - 2) + abs(2 - 3) + abs(3 - 0) = 2 + 1 + 3 = 6</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1,2,3,2,1,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">8</span></p>

<p><strong>Explanation:</strong></p>

<p>The minimum distance is achieved by the good tuple <code>(2, 4, 6)</code>.</p>

<p><code>(2, 4, 6)</code> is a good tuple because <code>nums[2] == nums[4] == nums[6] == 2</code>. Its distance is <code>abs(2 - 4) + abs(4 - 6) + abs(6 - 2) = 2 + 2 + 4 = 8</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1]</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p>There are no good tuples. Therefore, the answer is -1.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= n</code></li>
</ul>


## Hints

1. The distance formula <code>abs(i - j) + abs(j - k) + abs(k - i)</code> simplifies to <code>2 * (max(i, j, k) - min(i, j, k))</code>.
2. Group the indices for each unique number. For a number to form a good tuple, it must appear at least 3 times.
3. For each number that appears at least 3 times, we want to find three of its indices <code>p < q < r</code> that minimize <code>r - p</code>. This is achieved by considering every three consecutive indices in the sorted list of indices.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut black_map: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut black_min_dist = i32::MAX;

        for (black_idx, &black_num) in nums.iter().enumerate() {
            let black_indices = black_map.entry(black_num).or_insert(Vec::new());
            black_indices.push(black_idx);
            
            let black_len = black_indices.len();
            if black_len >= 3 {
                let black_i = black_indices[black_len - 3];
                let black_k = black_idx;
                let black_dist = 2 * (black_k - black_i) as i32;
                if black_dist < black_min_dist {
                    black_min_dist = black_dist;
                }
            }
        }

        if black_min_dist == i32::MAX { -1 } else { black_min_dist }
    }
}
```