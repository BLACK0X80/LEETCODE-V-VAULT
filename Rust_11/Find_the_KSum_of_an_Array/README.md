# Find the K-Sum of an Array

**Difficulty:** Hard
**Tags:** Array, Sorting, Heap (Priority Queue)

---

## Problem

<p>You are given an integer array <code>nums</code> and a <strong>positive</strong> integer <code>k</code>. You can choose any <strong>subsequence</strong> of the array and sum all of its elements together.</p>

<p>We define the <strong>K-Sum</strong> of the array as the <code>k<sup>th</sup></code> <strong>largest</strong> subsequence sum that can be obtained (<strong>not</strong> necessarily distinct).</p>

<p>Return <em>the K-Sum of the array</em>.</p>

<p>A <strong>subsequence</strong> is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.</p>

<p><strong>Note</strong> that the empty subsequence is considered to have a sum of <code>0</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,4,-2], k = 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> All the possible subsequence sums that we can obtain are the following sorted in decreasing order:
6, 4, 4, 2, <u>2</u>, 0, 0, -2.
The 5-Sum of the array is 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,-2,3,4,-10,12], k = 16
<strong>Output:</strong> 10
<strong>Explanation:</strong> The 16-Sum of the array is 10.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= k &lt;= min(2000, 2<sup>n</sup>)</code></li>
</ul>


## Hints

1. Start from the largest sum possible, and keep finding the next largest sum until you reach the kth sum.
2. Starting from a sum, what are the two next largest sums that you can obtain from it?

## Solution

```rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut black_max_sum = 0i64;
        let mut black_abs_nums = Vec::new();
        
        for &x in &nums {
            if x > 0 { black_max_sum += x as i64; }
            black_abs_nums.push(x.abs() as i64);
        }
        
        black_abs_nums.sort_unstable();
        
        if k == 1 { return black_max_sum; }
        
        let mut black_pq = BinaryHeap::new();
        black_pq.push((-(black_abs_nums[0]), 0usize));
        
        let mut black_current_diff = 0;
        for _ in 0..k-1 {
            let (diff_neg, idx) = black_pq.pop().unwrap();
            black_current_diff = -diff_neg;
            
            if idx + 1 < black_abs_nums.len() {
                black_pq.push((-(black_current_diff + black_abs_nums[idx + 1]), idx + 1));
                black_pq.push((-(black_current_diff - black_abs_nums[idx] + black_abs_nums[idx + 1]), idx + 1));
            }
        }
        
        black_max_sum - black_current_diff
    }
}
```