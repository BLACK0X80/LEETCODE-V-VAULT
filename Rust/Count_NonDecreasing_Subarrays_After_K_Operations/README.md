# Count Non-Decreasing Subarrays After K Operations

**Difficulty:** Hard
**Tags:** Array, Stack, Segment Tree, Queue, Sliding Window, Monotonic Stack, Monotonic Queue

---

## Problem

<p>You are given an array <code>nums</code> of <code>n</code> integers and an integer <code>k</code>.</p>

<p>For each subarray of <code>nums</code>, you can apply <strong>up to</strong> <code>k</code> operations on it. In each operation, you increment any element of the subarray by 1.</p>

<p><strong>Note</strong> that each subarray is considered independently, meaning changes made to one subarray do not persist to another.</p>

<p>Return the number of subarrays that you can make <strong>non-decreasing</strong> ​​​​​after performing at most <code>k</code> operations.</p>

<p>An array is said to be <strong>non-decreasing</strong> if each element is greater than or equal to its previous element, if it exists.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [6,3,1,2,4,4], k = 7</span></p>

<p><strong>Output:</strong> <span class="example-io">17</span></p>

<p><strong>Explanation:</strong></p>

<p>Out of all 21 possible subarrays of <code>nums</code>, only the subarrays <code>[6, 3, 1]</code>, <code>[6, 3, 1, 2]</code>, <code>[6, 3, 1, 2, 4]</code> and <code>[6, 3, 1, 2, 4, 4]</code> cannot be made non-decreasing after applying up to k = 7 operations. Thus, the number of non-decreasing subarrays is <code>21 - 4 = 17</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [6,3,1,3,6], k = 4</span></p>

<p><strong>Output:</strong> <span class="example-io">12</span></p>

<p><strong>Explanation:</strong></p>

<p>The subarray <code>[3, 1, 3, 6]</code> along with all subarrays of <code>nums</code> with three or fewer elements, except <code>[6, 3, 1]</code>, can be made non-decreasing after <code>k</code> operations. There are 5 subarrays of a single element, 4 subarrays of two elements, and 2 subarrays of three elements except <code>[6, 3, 1]</code>, so there are <code>1 + 5 + 4 + 2 = 12</code> subarrays that can be made non-decreasing.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Use a sparse table.
2. Compute <code>sp[e][i] = [lastElement, operations]</code> where <code>operations</code> is the number of <code>operations</code> required to make the subarray <code>nums[i...i + 2^e - 1]</code> non-decreasing, and <code>lastElement</code> be the value of the last element after the operations were applied on it.
3. How can we combine <code>sp[a][i]</code> with <code>sp[b][i + 2^a]</code> to find the answer for the subarray <code>nums[i...i + 2^a + 2^b - 1]</code>?

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn count_non_decreasing_subarrays(black_nums: Vec<i32>, black_k: i32) -> i64 {
        let mut black_res = 0i64;
        let mut black_window_head_indexes: VecDeque<usize> = VecDeque::new();
        let black_n = black_nums.len();
        let mut black_right = black_n as isize - 1;
        let mut black_k_remaining = black_k as i64;

        for black_left in (0..black_n).rev() {
            while !black_window_head_indexes.is_empty() && black_nums[*black_window_head_indexes.front().unwrap()] < black_nums[black_left] {
                let black_r_idx = black_window_head_indexes.pop_front().unwrap();
                let black_l_idx = if !black_window_head_indexes.is_empty() {
                    *black_window_head_indexes.front().unwrap()
                } else {
                    (black_right + 1) as usize
                };
                
                black_k_remaining -= (black_l_idx - black_r_idx) as i64 * (black_nums[black_left] - black_nums[black_r_idx]) as i64;
            }

            black_window_head_indexes.push_front(black_left);

            while black_k_remaining < 0 {
                black_k_remaining += (black_nums[*black_window_head_indexes.back().unwrap()] - black_nums[black_right as usize]) as i64;
                
                if *black_window_head_indexes.back().unwrap() == black_right as usize {
                    black_window_head_indexes.pop_back();
                }
                black_right -= 1;
            }

            black_res += (black_right - black_left as isize + 1) as i64;
        }

        black_res
    }
}
```