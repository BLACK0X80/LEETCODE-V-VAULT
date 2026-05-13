# Maximize Score After N Operations

**Difficulty:** Hard
**Tags:** Array, Math, Dynamic Programming, Backtracking, Bit Manipulation, Number Theory, Bitmask

---

## Problem

<p>You are given <code>nums</code>, an array of positive integers of size <code>2 * n</code>. You must perform <code>n</code> operations on this array.</p>

<p>In the <code>i<sup>th</sup></code> operation <strong>(1-indexed)</strong>, you will:</p>

<ul>
	<li>Choose two elements, <code>x</code> and <code>y</code>.</li>
	<li>Receive a score of <code>i * gcd(x, y)</code>.</li>
	<li>Remove <code>x</code> and <code>y</code> from <code>nums</code>.</li>
</ul>

<p>Return <em>the maximum score you can receive after performing </em><code>n</code><em> operations.</em></p>

<p>The function <code>gcd(x, y)</code> is the greatest common divisor of <code>x</code> and <code>y</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2]
<strong>Output:</strong> 1
<strong>Explanation:</strong>&nbsp;The optimal choice of operations is:
(1 * gcd(1, 2)) = 1
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,4,6,8]
<strong>Output:</strong> 11
<strong>Explanation:</strong>&nbsp;The optimal choice of operations is:
(1 * gcd(3, 6)) + (2 * gcd(4, 8)) = 3 + 8 = 11
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4,5,6]
<strong>Output:</strong> 14
<strong>Explanation:</strong>&nbsp;The optimal choice of operations is:
(1 * gcd(1, 5)) + (2 * gcd(2, 4)) + (3 * gcd(3, 6)) = 1 + 4 + 9 = 14
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 7</code></li>
	<li><code>nums.length == 2 * n</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Find every way to split the array until n groups of 2. Brute force recursion is acceptable.
2. Calculate the gcd of every pair and greedily multiply the largest gcds.

## Solution

```rust
impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut black_gcd = vec![vec![0; n]; n];
        for i in 0..n { for j in 0..n {
            let mut a = nums[i]; let mut b = nums[j];
            while b != 0 { let t = b; b = a%b; a = t; }
            black_gcd[i][j] = a;
        }}
        let mut black_dp = vec![0; 1<<n];
        for mask in 0..(1<<n) {
            let cnt = (mask as u32).count_ones() as i32;
            if cnt % 2 != 0 { continue; }
            let op = cnt/2 + 1;
            for i in 0..n { if mask & (1<<i) != 0 { continue; }
                for j in i+1..n { if mask & (1<<j) != 0 { continue; }
                    let nm = mask | (1<<i) | (1<<j);
                    black_dp[nm] = black_dp[nm].max(black_dp[mask] + op * black_gcd[i][j]);
                }
            }
        }
        black_dp[(1<<n)-1]
    }
}
```