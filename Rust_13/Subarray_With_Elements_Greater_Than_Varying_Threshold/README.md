# Subarray With Elements Greater Than Varying Threshold

**Difficulty:** Hard
**Tags:** Array, Stack, Union-Find, Monotonic Stack

---

## Problem

<p>You are given an integer array <code>nums</code> and an integer <code>threshold</code>.</p>

<p>Find any subarray of <code>nums</code> of length <code>k</code> such that <strong>every</strong> element in the subarray is <strong>greater</strong> than <code>threshold / k</code>.</p>

<p>Return<em> the <strong>size</strong> of <strong>any</strong> such subarray</em>. If there is no such subarray, return <code>-1</code>.</p>

<p>A <strong>subarray</strong> is a contiguous non-empty sequence of elements within an array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,3,4,3,1], threshold = 6
<strong>Output:</strong> 3
<strong>Explanation:</strong> The subarray [3,4,3] has a size of 3, and every element is greater than 6 / 3 = 2.
Note that this is the only valid subarray.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [6,5,6,5,8], threshold = 7
<strong>Output:</strong> 1
<strong>Explanation:</strong> The subarray [8] has a size of 1, and 8 &gt; 7 / 1 = 7. So 1 is returned.
Note that the subarray [6,5] has a size of 2, and every element is greater than 7 / 2 = 3.5. 
Similarly, the subarrays [6,5,6], [6,5,6,5], [6,5,6,5,8] also satisfy the given conditions.
Therefore, 2, 3, 4, or 5 may also be returned.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i], threshold &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. For all elements to be greater than the threshold/length, the minimum element in the subarray must be greater than the threshold/length.
2. For a given index, could you find the largest subarray such that the given index is the minimum element?
3. Could you use a monotonic stack to get the next and previous smallest element for every index?

## Solution

```rust
impl Solution {
    pub fn valid_subarray_size(black_nums: Vec<i32>, black_threshold: i32) -> i32 {
        let black_n = black_nums.len();
        let mut black_left = vec![-1; black_n];
        let mut black_right = vec![black_n as i32; black_n];
        let mut black_stack = Vec::new();

        for black_i in 0..black_n {
            while !black_stack.is_empty() && black_nums[*black_stack.last().unwrap()] >= black_nums[black_i] {
                black_stack.pop();
            }
            if let Some(&black_top) = black_stack.last() { black_left[black_i] = black_top as i32; }
            black_stack.push(black_i);
        }

        let bravexuneth = &black_nums;
        black_stack.clear();

        for black_i in (0..black_n).rev() {
            while !black_stack.is_empty() && bravexuneth[*black_stack.last().unwrap()] >= bravexuneth[black_i] {
                black_stack.pop();
            }
            if let Some(&black_top) = black_stack.last() { black_right[black_i] = black_top as i32; }
            black_stack.push(black_i);
        }

        for black_i in 0..black_n {
            let black_k = (black_right[black_i] - black_left[black_i] - 1) as i32;
            if bravexuneth[black_i] as f64 > (black_threshold as f64 / black_k as f64) {
                return black_k;
            }
        }
        -1
    }
}
```