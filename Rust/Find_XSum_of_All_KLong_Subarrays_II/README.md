# Find X-Sum of All K-Long Subarrays II

**Difficulty:** Hard
**Tags:** Array, Hash Table, Sliding Window, Heap (Priority Queue)

---

## Problem

<p>You are given an array <code>nums</code> of <code>n</code> integers and two integers <code>k</code> and <code>x</code>.</p>

<p>The <strong>x-sum</strong> of an array is calculated by the following procedure:</p>

<ul>
	<li>Count the occurrences of all elements in the array.</li>
	<li>Keep only the occurrences of the top <code>x</code> most frequent elements. If two elements have the same number of occurrences, the element with the <strong>bigger</strong> value is considered more frequent.</li>
	<li>Calculate the sum of the resulting array.</li>
</ul>

<p><strong>Note</strong> that if an array has less than <code>x</code> distinct elements, its <strong>x-sum</strong> is the sum of the array.</p>

<p>Return an integer array <code>answer</code> of length <code>n - k + 1</code> where <code>answer[i]</code> is the <strong>x-sum</strong> of the <span data-keyword="subarray-nonempty">subarray</span> <code>nums[i..i + k - 1]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1,2,2,3,4,2,3], k = 6, x = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">[6,10,12]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>For subarray <code>[1, 1, 2, 2, 3, 4]</code>, only elements 1 and 2 will be kept in the resulting array. Hence, <code>answer[0] = 1 + 1 + 2 + 2</code>.</li>
	<li>For subarray <code>[1, 2, 2, 3, 4, 2]</code>, only elements 2 and 4 will be kept in the resulting array. Hence, <code>answer[1] = 2 + 2 + 2 + 4</code>. Note that 4 is kept in the array since it is bigger than 3 and 1 which occur the same number of times.</li>
	<li>For subarray <code>[2, 2, 3, 4, 2, 3]</code>, only elements 2 and 3 are kept in the resulting array. Hence, <code>answer[2] = 2 + 2 + 2 + 3 + 3</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,8,7,8,7,5], k = 2, x = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">[11,15,15,15,12]</span></p>

<p><strong>Explanation:</strong></p>

<p>Since <code>k == x</code>, <code>answer[i]</code> is equal to the sum of the subarray <code>nums[i..i + k - 1]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>nums.length == n</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= x &lt;= k &lt;= nums.length</code></li>
</ul>


## Hints

1. Use sliding window.
2. Use two sets ordered by frequency. One of the sets will only contain the top <code>x</code> frequent elements, and the second will contain all other elements.
3. Update the two sets whenever you slide the window, and maintain a sum of the elements in the set with <code>x</code> elements

## Solution

```rust
use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn find_x_sum(black_nums: Vec<i32>, black_k: i32, black_x: i32) -> Vec<i64> {
        let black_n = black_nums.len();
        let mut black_counts = HashMap::new();
        let mut black_top_x = BTreeSet::new();
        let mut black_others = BTreeSet::new();
        let mut black_current_x_sum = 0i64;
        let mut black_res = Vec::new();

        let mut black_update = |black_val: i32, black_delta: i32, black_top_x: &mut BTreeSet<(i32, i32)>, black_others: &mut BTreeSet<(i32, i32)>, black_counts: &mut HashMap<i32, i32>, black_current_x_sum: &mut i64, black_x: i32| {
            let black_old_count = *black_counts.get(&black_val).unwrap_or(&0);
            if black_old_count > 0 {
                let black_pair = (black_old_count, black_val);
                if black_top_x.contains(&black_pair) {
                    black_top_x.remove(&black_pair);
                    *black_current_x_sum -= black_old_count as i64 * black_val as i64;
                } else {
                    black_others.remove(&black_pair);
                }
            }

            let black_new_count = black_old_count + black_delta;
            if black_new_count > 0 {
                black_counts.insert(black_val, black_new_count);
                black_others.insert((black_new_count, black_val));
            } else {
                black_counts.remove(&black_val);
            }

            while !black_others.is_empty() && (black_top_x.len() < black_x as usize || black_others.last().unwrap() > black_top_x.first().unwrap()) {
                black_top_x.insert(*black_others.last().unwrap());
                let black_added = black_others.pop_last().unwrap();
                *black_current_x_sum += black_added.0 as i64 * black_added.1 as i64;

                if black_top_x.len() > black_x as usize {
                    let black_removed = *black_top_x.first().unwrap();
                    black_top_x.remove(&black_removed);
                    *black_current_x_sum -= black_removed.0 as i64 * black_removed.1 as i64;
                    black_others.insert(black_removed);
                }
            }
        };

        for i in 0..black_n {
            black_update(black_nums[i], 1, &mut black_top_x, &mut black_others, &mut black_counts, &mut black_current_x_sum, black_x);
            if i >= black_k as usize {
                black_update(black_nums[i - black_k as usize], -1, &mut black_top_x, &mut black_others, &mut black_counts, &mut black_current_x_sum, black_x);
            }
            if i >= (black_k - 1) as usize {
                black_res.push(black_current_x_sum);
            }
        }
        black_res
    }
}
```