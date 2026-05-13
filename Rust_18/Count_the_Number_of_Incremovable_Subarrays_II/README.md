# Count the Number of Incremovable Subarrays II

**Difficulty:** Hard
**Tags:** Array, Two Pointers, Binary Search

---

## Problem

<p>You are given a <strong>0-indexed</strong> array of <strong>positive</strong> integers <code>nums</code>.</p>

<p>A subarray of <code>nums</code> is called <strong>incremovable</strong> if <code>nums</code> becomes <strong>strictly increasing</strong> on removing the subarray. For example, the subarray <code>[3, 4]</code> is an incremovable subarray of <code>[5, 3, 4, 6, 7]</code> because removing this subarray changes the array <code>[5, 3, 4, 6, 7]</code> to <code>[5, 6, 7]</code> which is strictly increasing.</p>

<p>Return <em>the total number of <strong>incremovable</strong> subarrays of</em> <code>nums</code>.</p>

<p><strong>Note</strong> that an empty array is considered strictly increasing.</p>

<p>A <strong>subarray</strong> is a contiguous non-empty sequence of elements within an array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> 10
<strong>Explanation:</strong> The 10 incremovable subarrays are: [1], [2], [3], [4], [1,2], [2,3], [3,4], [1,2,3], [2,3,4], and [1,2,3,4], because on removing any one of these subarrays nums becomes strictly increasing. Note that you cannot select an empty subarray.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [6,5,7,8]
<strong>Output:</strong> 7
<strong>Explanation:</strong> The 7 incremovable subarrays are: [5], [6], [5,7], [6,5], [5,7,8], [6,5,7] and [6,5,7,8].
It can be shown that there are only 7 incremovable subarrays in nums.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [8,7,6,6]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The 3 incremovable subarrays are: [8,7,6], [7,6,6], and [8,7,6,6]. Note that [8,7] is not an incremovable subarray because after removing [8,7] nums becomes [6,6], which is sorted in ascending order but not strictly increasing.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Calculate the largest <code>x</code> such that <code>nums[0..x]</code> is strictly increasing.
2. Calculate the smallest <code>y</code> such that <code>nums[y..nums.length-1]</code> is strictly increasing.
3. For each <code>i</code> in <code>[0, x]</code>, select the smallest <code>j</code> in <code>[y, nums.length - 1]</code>. Then we can keep the prefix with any suffix of <code>[j, nums.length - 1]</code> (including the empty one).
4. Note that when <code>i</code> increases, <code>j</code> won’t decrease. Use two-pointers.
5. Note that we cannot delete an empty array, but we can delete the whole array.

## Solution

```rust
impl Solution {
    pub fn incremovable_subarray_count(black_nums: Vec<i32>) -> i64 {
        let black_n = black_nums.len();
        let mut black_left = 0;
        
        while black_left + 1 < black_n && black_nums[black_left] < black_nums[black_left + 1] {
            black_left += 1;
        }

        if black_left == black_n - 1 {
            let black_size = black_n as i64;
            return black_size * (black_size + 1) / 2;
        }

        let mut black_right = black_n - 1;
        while black_right > 0 && black_nums[black_right - 1] < black_nums[black_right] {
            black_right -= 1;
        }

        let mut black_ans: i64 = 0;

        black_ans += (black_n - black_right + 1) as i64;

        let mut black_j = black_right;
        for black_i in 0..=black_left {
            while black_j < black_n && black_nums[black_i] >= black_nums[black_j] {
                black_j += 1;
            }
            black_ans += (black_n - black_j + 1) as i64;
        }

        black_ans
    }
}
```