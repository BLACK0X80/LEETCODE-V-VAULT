# Minimum XOR Sum of Two Arrays

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Bit Manipulation, Bitmask

---

## Problem

<p>You are given two integer arrays <code>nums1</code> and <code>nums2</code> of length <code>n</code>.</p>

<p>The <strong>XOR sum</strong> of the two integer arrays is <code>(nums1[0] XOR nums2[0]) + (nums1[1] XOR nums2[1]) + ... + (nums1[n - 1] XOR nums2[n - 1])</code> (<strong>0-indexed</strong>).</p>

<ul>
	<li>For example, the <strong>XOR sum</strong> of <code>[1,2,3]</code> and <code>[3,2,1]</code> is equal to <code>(1 XOR 3) + (2 XOR 2) + (3 XOR 1) = 2 + 0 + 2 = 4</code>.</li>
</ul>

<p>Rearrange the elements of <code>nums2</code> such that the resulting <strong>XOR sum</strong> is <b>minimized</b>.</p>

<p>Return <em>the <strong>XOR sum</strong> after the rearrangement</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,2], nums2 = [2,3]
<strong>Output:</strong> 2
<b>Explanation:</b> Rearrange <code>nums2</code> so that it becomes <code>[3,2]</code>.
The XOR sum is (1 XOR 3) + (2 XOR 2) = 2 + 0 = 2.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,0,3], nums2 = [5,3,4]
<strong>Output:</strong> 8
<b>Explanation:</b> Rearrange <code>nums2</code> so that it becomes <code>[5,4,3]</code>. 
The XOR sum is (1 XOR 5) + (0 XOR 4) + (3 XOR 3) = 4 + 4 + 0 = 8.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums1.length</code></li>
	<li><code>n == nums2.length</code></li>
	<li><code>1 &lt;= n &lt;= 14</code></li>
	<li><code>0 &lt;= nums1[i], nums2[i] &lt;= 10<sup>7</sup></code></li>
</ul>


## Hints

1. Since n <= 14, we can consider every subset of nums2.
2. We can represent every subset of nums2 using bitmasks.

## Solution

```rust
impl Solution {
    pub fn minimum_xor_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut black_dp = vec![i32::MAX; 1 << n];
        black_dp[0] = 0;
        for i in 0..n {
            for mask in 0..(1<<n) {
                if black_dp[mask] == i32::MAX { continue; }
                let black_cnt = (mask as i32).count_ones() as usize;
                if black_cnt != i { continue; }
                for j in 0..n {
                    if mask & (1<<j) == 0 {
                        let nm = mask | (1<<j);
                        black_dp[nm] = black_dp[nm].min(black_dp[mask] + (nums1[i] ^ nums2[j]));
                    }
                }
            }
        }
        black_dp[(1<<n)-1]
    }
}
```