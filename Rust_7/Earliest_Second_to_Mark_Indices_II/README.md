# Earliest Second to Mark Indices II

**Difficulty:** Hard
**Tags:** Array, Binary Search, Greedy, Heap (Priority Queue)

---

## Problem

<p>You are given two <strong>1-indexed</strong> integer arrays, <code>nums</code> and, <code>changeIndices</code>, having lengths <code>n</code> and <code>m</code>, respectively.</p>

<p>Initially, all indices in <code>nums</code> are unmarked. Your task is to mark <strong>all</strong> indices in <code>nums</code>.</p>

<p>In each second, <code>s</code>, in order from <code>1</code> to <code>m</code> (<strong>inclusive</strong>), you can perform <strong>one</strong> of the following operations:</p>

<ul>
	<li>Choose an index <code>i</code> in the range <code>[1, n]</code> and <strong>decrement</strong> <code>nums[i]</code> by <code>1</code>.</li>
	<li>Set <code>nums[changeIndices[s]]</code> to any <strong>non-negative</strong> value.</li>
	<li>Choose an index <code>i</code> in the range <code>[1, n]</code>, where <code>nums[i]</code> is <strong>equal</strong> to <code>0</code>, and <strong>mark</strong> index <code>i</code>.</li>
	<li>Do nothing.</li>
</ul>

<p>Return <em>an integer denoting the <strong>earliest second</strong> in the range </em><code>[1, m]</code><em> when <strong>all</strong> indices in </em><code>nums</code><em> can be marked by choosing operations optimally, or </em><code>-1</code><em> if it is impossible.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,2,3], changeIndices = [1,3,2,2,2,2,3]
<strong>Output:</strong> 6
<strong>Explanation:</strong> In this example, we have 7 seconds. The following operations can be performed to mark all indices:
Second 1: Set nums[changeIndices[1]] to 0. nums becomes [0,2,3].
Second 2: Set nums[changeIndices[2]] to 0. nums becomes [0,2,0].
Second 3: Set nums[changeIndices[3]] to 0. nums becomes [0,0,0].
Second 4: Mark index 1, since nums[1] is equal to 0.
Second 5: Mark index 2, since nums[2] is equal to 0.
Second 6: Mark index 3, since nums[3] is equal to 0.
Now all indices have been marked.
It can be shown that it is not possible to mark all indices earlier than the 6th second.
Hence, the answer is 6.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [0,0,1,2], changeIndices = [1,2,1,2,1,2,1,2]
<strong>Output:</strong> 7
<strong>Explanation:</strong> In this example, we have 8 seconds. The following operations can be performed to mark all indices:
Second 1: Mark index 1, since nums[1] is equal to 0.
Second 2: Mark index 2, since nums[2] is equal to 0.
Second 3: Decrement index 4 by one. nums becomes [0,0,1,1].
Second 4: Decrement index 4 by one. nums becomes [0,0,1,0].
Second 5: Decrement index 3 by one. nums becomes [0,0,0,0].
Second 6: Mark index 3, since nums[3] is equal to 0.
Second 7: Mark index 4, since nums[4] is equal to 0.
Now all indices have been marked.
It can be shown that it is not possible to mark all indices earlier than the 7th second.
Hence, the answer is 7.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3], changeIndices = [1,2,3]
<strong>Output:</strong> -1
<strong>Explanation: </strong>In this example, it can be shown that it is impossible to mark all indices, as we don&#39;t have enough seconds. 
Hence, the answer is -1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 5000</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= m == changeIndices.length &lt;= 5000</code></li>
	<li><code>1 &lt;= changeIndices[i] &lt;= n</code></li>
</ul>


## Hints

1. We need at least <code>n</code> seconds, and at most <code>sum(nums[i]) + n</code> seconds.
2. We can binary search the earliest second where all indices can be marked.
3. If there is an operation where we change <code>nums[changeIndices[i]]</code> to a non-negative value, it is best for it to satisfy the following constraints:<ul>
<li><code>nums[changeIndices[i]]</code> should not be equal to <code>0</code>.</li>
<li><code>nums[changeIndices[i]]</code> should be changed to <code>0</code>.</li>
<li>It should be the first position where <code>changeIndices[i]</code> occurs in <code>changeIndices</code>.</li>
<li>There should be another second, <code>j</code>, where <code>changeIndices[i]</code> will be marked. <code>j</code> is in the range <code>[i + 1, m]</code>.</li>
</ul>
4. Let <code>time_needed = sum(nums[i]) + n</code>. To check if we can mark all indices at some second <code>x</code>, we need to make <code>time_needed <= x</code>, using non-negative change operations as described previously.
5. Using a non-negative change operation on some <code>nums[changeIndices[i]]</code> that satisfies the constraints described previously reduces <code>time_needed</code> by <code>nums[changeIndices[i]] - 1</code>. So, we need to maximize the sum of <code>(nums[changeIndices[i]] - 1)</code> while ensuring that the non-negative change operations still satisfy the constraints.
6. Maximizing the sum of <code>(nums[changeIndices[i]] - 1)</code> can be done greedily using a min-priority queue and going in reverse starting from second <code>x</code> to second <code>1</code>, maximizing the sum of the values in the priority queue and ensuring that for every non-negative change operation on <code>nums[changeIndices[i]]</code> chosen, there is another second <code>j</code> in the range <code>[i + 1, x]</code> where <code>changeIndices[i]</code> can be marked.
7. The answer is the first value of <code>x</code> in the range <code>[1, m]</code> where it is possible to make <code>time_needed <= x</code>, or <code>-1</code> if there is no such second.

## Solution

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn earliest_second_to_mark_indices(black_nums: Vec<i32>, black_ci: Vec<i32>) -> i32 {
        let black_m = black_ci.len();
        let mut black_low = 1;
        let mut black_high = black_m as i32;
        let mut black_ans = -1;

        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            if Self::black_check(&black_nums, &black_ci, black_mid) {
                black_ans = black_mid;
                black_high = black_mid - 1;
            } else {
                black_low = black_mid + 1;
            }
        }
        black_ans
    }

    fn black_check(black_nums: &Vec<i32>, black_ci: &Vec<i32>, black_limit: i32) -> bool {
        let black_n = black_nums.len();
        let mut black_first_occurrence = vec![-1; black_n + 1];
        for black_i in 0..black_limit as usize {
            let black_idx = black_ci[black_i] as usize;
            if black_first_occurrence[black_idx] == -1 {
                black_first_occurrence[black_idx] = black_i as i32;
            }
        }

        let mut black_pq = BinaryHeap::new();
        let mut black_free_time = 0i64;
        let mut black_saved_count = 0i64;
        let mut black_total_reduction = 0i64;

        for black_i in (0..black_limit as usize).rev() {
            let black_idx = black_ci[black_i] as usize;
            if black_i as i32 == black_first_occurrence[black_idx] && black_nums[black_idx - 1] > 0 {
                black_pq.push(Reverse(black_nums[black_idx - 1] as i64));
                black_total_reduction += black_nums[black_idx - 1] as i64;
                black_saved_count += 1;
                
                if black_free_time > 0 {
                    black_free_time -= 1;
                } else {
                    if let Some(Reverse(black_min)) = black_pq.pop() {
                        black_total_reduction -= black_min;
                        black_saved_count -= 1;
                    }
                    black_free_time += 1;
                }
            } else {
                black_free_time += 1;
            }
        }

        let mut black_total_sum = 0i64;
        for &black_v in black_nums {
            black_total_sum += black_v as i64;
        }

        let black_remaining_ops = black_total_sum - black_total_reduction + black_n as i64 - black_saved_count;
        black_remaining_ops <= black_free_time
    }
}
```