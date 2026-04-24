# Count the Number of Square-Free Subsets

**Difficulty:** Medium
**Tags:** Array, Math, Dynamic Programming, Bit Manipulation, Number Theory, Bitmask

---

## Problem

<p>You are given a positive integer <strong>0-indexed</strong>&nbsp;array <code>nums</code>.</p>

<p>A subset of the array <code>nums</code> is <strong>square-free</strong> if the product of its elements is a <strong>square-free integer</strong>.</p>

<p>A <strong>square-free integer</strong> is an integer that is divisible by no square number other than <code>1</code>.</p>

<p>Return <em>the number of square-free non-empty subsets of the array</em> <strong>nums</strong>. Since the answer may be too large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>A <strong>non-empty</strong>&nbsp;<strong>subset</strong> of <code>nums</code> is an array that can be obtained by deleting some (possibly none but not all) elements from <code>nums</code>. Two subsets are different if and only if the chosen indices to delete are different.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,4,4,5]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are 3 square-free subsets in this example:
- The subset consisting of the 0<sup>th</sup> element [3]. The product of its elements is 3, which is a square-free integer.
- The subset consisting of the 3<sup>rd</sup> element [5]. The product of its elements is 5, which is a square-free integer.
- The subset consisting of 0<sup>th</sup> and 3<sup>rd</sup> elements [3,5]. The product of its elements is 15, which is a square-free integer.
It can be proven that there are no more than 3 square-free subsets in the given array.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is 1 square-free subset in this example:
- The subset consisting of the 0<sup>th</sup> element [1]. The product of its elements is 1, which is a square-free integer.
It can be proven that there is no more than 1 square-free subset in the given array.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length&nbsp;&lt;= 1000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 30</code></li>
</ul>


## Hints

1. There are 10 primes before number 30.
2. Label primes from {2, 3, … 29} with {0,1, … 9} and let DP(i, mask) denote the number of subsets before index: i with the subset of taken primes: mask.
3. If the mask and prime factorization of nums[i] have a common prime, then it is impossible to add to the current subset, otherwise, it is possible.

## Solution

```rust
impl Solution {
    pub fn square_free_subsets(black_nums: Vec<i32>) -> i32 {
        let black_p = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]; let mut black_dp = vec![0i64; 1 << 10]; black_dp[0] = 1;
        for mut black_x in black_nums { let mut black_mask = 0; let mut black_ok = true; for black_i in 0..10 { if black_x % (black_p[black_i] * black_p[black_i]) == 0 { black_ok = false; break; } if black_x % black_p[black_i] == 0 { black_mask |= 1 << black_i; } } if black_ok { for black_m in (0..1 << 10).rev() { if (black_m & black_mask) == black_mask { black_dp[black_m] = (black_dp[black_m] + black_dp[black_m ^ black_mask]) % 1_000_000_007; } } } }
        ((black_dp.iter().sum::<i64>() - 1 + 1_000_000_007) % 1_000_000_007) as i32
    }
}
```