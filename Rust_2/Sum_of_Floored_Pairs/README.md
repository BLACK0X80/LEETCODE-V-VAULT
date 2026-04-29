# Sum of Floored Pairs

**Difficulty:** Hard
**Tags:** Array, Math, Binary Search, Counting, Enumeration, Prefix Sum

---

## Problem

<p>Given an integer array <code>nums</code>, return the sum of <code>floor(nums[i] / nums[j])</code> for all pairs of indices <code>0 &lt;= i, j &lt; nums.length</code> in the array. Since the answer may be too large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>The <code>floor()</code> function returns the integer part of the division.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,5,9]
<strong>Output:</strong> 10
<strong>Explanation:</strong>
floor(2 / 5) = floor(2 / 9) = floor(5 / 9) = 0
floor(2 / 2) = floor(5 / 5) = floor(9 / 9) = 1
floor(5 / 2) = 2
floor(9 / 2) = 4
floor(9 / 5) = 1
We calculate the floor of the division for every pair of indices in the array then sum them up.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [7,7,7,7,7,7,7]
<strong>Output:</strong> 49
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Find the frequency (number of occurrences) of all elements in the array.
2. For each element, iterate through its multiples and multiply frequencies to find the answer.

## Solution

```rust
impl Solution {
    pub fn sum_of_floored_pairs(black1: Vec<i32>) -> i32 {
        let black2 = 100001;
        let mut black3 = vec![0i64; black2];
        for &black4 in &black1 { black3[black4 as usize] += 1; }
        let mut black5 = vec![0i64; black2];
        for black6 in 1..black2 { black5[black6] = black5[black6 - 1] + black3[black6]; }
        let mut black7 = 0;
        let black8 = 1_000_000_007;
        for black9 in 1..black2 {
            if black3[black9] == 0 { continue; }
            for black10 in (black9..black2).step_by(black9) {
                let black11 = std::cmp::min(black10 + black9 - 1, black2 - 1);
                let black12 = black5[black11] - black5[black10 - 1];
                black7 = (black7 + black3[black9] * black12 * (black10 / black9) as i64) % black8;
            }
        }
        black7 as i32
    }
}
```