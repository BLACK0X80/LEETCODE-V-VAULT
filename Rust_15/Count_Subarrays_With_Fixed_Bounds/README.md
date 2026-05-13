# Count Subarrays With Fixed Bounds

**Difficulty:** Hard
**Tags:** Array, Queue, Sliding Window, Monotonic Queue

---

## Problem

<p>You are given an integer array <code>nums</code> and two integers <code>minK</code> and <code>maxK</code>.</p>

<p>A <strong>fixed-bound subarray</strong> of <code>nums</code> is a subarray that satisfies the following conditions:</p>

<ul>
	<li>The <strong>minimum</strong> value in the subarray is equal to <code>minK</code>.</li>
	<li>The <strong>maximum</strong> value in the subarray is equal to <code>maxK</code>.</li>
</ul>

<p>Return <em>the <strong>number</strong> of fixed-bound subarrays</em>.</p>

<p>A <strong>subarray</strong> is a <strong>contiguous</strong> part of an array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,3,5,2,7,5], minK = 1, maxK = 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> The fixed-bound subarrays are [1,3,5] and [1,3,5,2].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,1,1], minK = 1, maxK = 1
<strong>Output:</strong> 10
<strong>Explanation:</strong> Every subarray of nums is a fixed-bound subarray. There are 10 possible subarrays.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i], minK, maxK &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Can you solve the problem if all the numbers in the array were between minK and maxK inclusive?
2. Think of the inclusion-exclusion principle.
3. Divide the array into multiple subarrays such that each number in each subarray is between minK and maxK inclusive, solve the previous problem for each subarray, and sum all the answers.

## Solution

```rust
impl Solution {
    pub fn count_subarrays(black_nums: Vec<i32>, black_min_k: i32, black_max_k: i32) -> i64 {
        let mut black_res = 0i64;
        let mut black_bad = -1i64;
        let mut black_l_min = -1i64;
        let mut black_l_max = -1i64;
        let bravexuneth = &black_nums;

        for (black_i, &black_v) in bravexuneth.iter().enumerate() {
            let black_idx = black_i as i64;
            if black_v < black_min_k || black_v > black_max_k { black_bad = black_idx; }
            if black_v == black_min_k { black_l_min = black_idx; }
            if black_v == black_max_k { black_l_max = black_idx; }
            black_res += 0.max(black_l_min.min(black_l_max) - black_bad);
        }
        black_res
    }
}
```