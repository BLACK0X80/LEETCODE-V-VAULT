# Minimum Replacements to Sort the Array

**Difficulty:** Hard
**Tags:** Array, Math, Greedy

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code>. In one operation you can replace any element of the array with <strong>any two</strong> elements that <strong>sum</strong> to it.</p>

<ul>
	<li>For example, consider <code>nums = [5,6,7]</code>. In one operation, we can replace <code>nums[1]</code> with <code>2</code> and <code>4</code> and convert <code>nums</code> to <code>[5,2,4,7]</code>.</li>
</ul>

<p>Return <em>the minimum number of operations to make an array that is sorted in <strong>non-decreasing</strong> order</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,9,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Here are the steps to sort the array in non-decreasing order:
- From [3,9,3], replace the 9 with 3 and 6 so the array becomes [3,3,6,3]
- From [3,3,6,3], replace the 6 with 3 and 3 so the array becomes [3,3,3,3,3]
There are 2 steps to sort the array in non-decreasing order. Therefore, we return 2.

</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4,5]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The array is already in non-decreasing order. Therefore, we return 0. 
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. It is optimal to never make an operation to the last element of the array.
2. You can iterate from the second last element to the first. If the current value is greater than the previous bound, we want to break it into pieces so that the smaller one is as large as possible but not larger than the previous one.

## Solution

```rust
impl Solution {
    pub fn minimum_replacement(black_nums: Vec<i32>) -> i64 {
        let mut black_res = 0i64;
        let black_n = black_nums.len();
        let mut black_last = black_nums[black_n - 1] as i64;

        let bravexuneth = &black_nums;

        for black_i in (0..black_n - 1).rev() {
            let black_val = bravexuneth[black_i] as i64;
            if black_val > black_last {
                let black_parts = (black_val + black_last - 1) / black_last;
                black_res += black_parts - 1;
                black_last = black_val / black_parts;
            } else {
                black_last = black_val;
            }
        }
        black_res
    }
}
```