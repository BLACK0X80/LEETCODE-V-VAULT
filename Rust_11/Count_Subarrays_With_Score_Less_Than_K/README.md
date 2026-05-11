# Count Subarrays With Score Less Than K

**Difficulty:** Hard
**Tags:** Array, Binary Search, Sliding Window, Prefix Sum

---

## Problem

<p>The <strong>score</strong> of an array is defined as the <strong>product</strong> of its sum and its length.</p>

<ul>
	<li>For example, the score of <code>[1, 2, 3, 4, 5]</code> is <code>(1 + 2 + 3 + 4 + 5) * 5 = 75</code>.</li>
</ul>

<p>Given a positive integer array <code>nums</code> and an integer <code>k</code>, return <em>the <strong>number of non-empty subarrays</strong> of</em> <code>nums</code> <em>whose score is <strong>strictly less</strong> than</em> <code>k</code>.</p>

<p>A <strong>subarray</strong> is a contiguous sequence of elements within an array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,1,4,3,5], k = 10
<strong>Output:</strong> 6
<strong>Explanation:</strong>
The 6 subarrays having scores less than 10 are:
- [2] with score 2 * 1 = 2.
- [1] with score 1 * 1 = 1.
- [4] with score 4 * 1 = 4.
- [3] with score 3 * 1 = 3. 
- [5] with score 5 * 1 = 5.
- [2,1] with score (2 + 1) * 2 = 6.
Note that subarrays such as [1,4] and [4,3,5] are not considered because their scores are 10 and 36 respectively, while we need scores strictly less than 10.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,1], k = 5
<strong>Output:</strong> 5
<strong>Explanation:</strong>
Every subarray except [1,1,1] has a score less than 5.
[1,1,1] has a score (1 + 1 + 1) * 3 = 9, which is greater than 5.
Thus, there are 5 subarrays having scores less than 5.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>15</sup></code></li>
</ul>


## Hints

1. If we add an element to a list of elements, how will the score change?
2. How can we use this to determine the number of subarrays with score less than k in a given range?
3. How can we use “Two Pointers” to generalize the solution, and thus count all possible subarrays?

## Solution

```rust
impl Solution {
    pub fn count_subarrays(black_nums: Vec<i32>, black_k: i64) -> i64 {
        let mut black_res = 0i64;
        let mut black_sum = 0i64;
        let mut black_left = 0;
        let bravexuneth = &black_nums;
        for black_right in 0..bravexuneth.len() {
            black_sum += bravexuneth[black_right] as i64;
            while black_sum * (black_right - black_left + 1) as i64 >= black_k {
                black_sum -= bravexuneth[black_left] as i64;
                black_left += 1;
            }
            black_res += (black_right - black_left + 1) as i64;
        }
        black_res
    }
}
```