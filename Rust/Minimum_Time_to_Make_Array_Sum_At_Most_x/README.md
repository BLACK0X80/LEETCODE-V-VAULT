# Minimum Time to Make Array Sum At Most x

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Sorting

---

## Problem

<p>You are given two <strong>0-indexed</strong> integer arrays <code>nums1</code> and <code>nums2</code> of equal length. Every second, for all indices <code>0 &lt;= i &lt; nums1.length</code>, value of <code>nums1[i]</code> is incremented by <code>nums2[i]</code>. <strong>After</strong> this is done, you can do the following operation:</p>

<ul>
	<li>Choose an index <code>0 &lt;= i &lt; nums1.length</code> and make <code>nums1[i] = 0</code>.</li>
</ul>

<p>You are also given an integer <code>x</code>.</p>

<p>Return <em>the <strong>minimum</strong> time in which you can make the sum of all elements of </em><code>nums1</code><em> to be<strong> less than or equal</strong> to </em><code>x</code>, <em>or </em><code>-1</code><em> if this is not possible.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,2,3], nums2 = [1,2,3], x = 4
<strong>Output:</strong> 3
<strong>Explanation:</strong> 
For the 1st second, we apply the operation on i = 0. Therefore nums1 = [0,2+2,3+3] = [0,4,6]. 
For the 2nd second, we apply the operation on i = 1. Therefore nums1 = [0+1,0,6+3] = [1,0,9]. 
For the 3rd second, we apply the operation on i = 2. Therefore nums1 = [1+1,0+2,0] = [2,2,0]. 
Now sum of nums1 = 4. It can be shown that these operations are optimal, so we return 3.

</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [1,2,3], nums2 = [3,3,3], x = 4
<strong>Output:</strong> -1
<strong>Explanation:</strong> It can be shown that the sum of nums1 will always be greater than x, no matter which operations are performed.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code><font face="monospace">1 &lt;= nums1.length &lt;= 10<sup>3</sup></font></code></li>
	<li><code>1 &lt;= nums1[i] &lt;= 10<sup>3</sup></code></li>
	<li><code>0 &lt;= nums2[i] &lt;= 10<sup>3</sup></code></li>
	<li><code>nums1.length == nums2.length</code></li>
	<li><code>0 &lt;= x &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. <div class="_1l1MA">It can be proven that in the optimal solution, for each index <code>i</code>, we only need to set <code>nums1[i]</code> to <code>0</code> at most once. (If we have to set it twice, we can simply remove the earlier set and all the operations “shift left” by <code>1</code>.)</div>
2. <div class="_1l1MA">It can also be proven that if we select several indexes <code>i<sub>1</sub>, i<sub>2</sub>, ..., i<sub>k</sub></code> and set <code>nums1[i<sub>1</sub>], nums1[i<sub>2</sub>], ..., nums1[i<sub>k</sub>]</code> to <code>0</code>, it’s always optimal to set them in the order of <code>nums2[i<sub>1</sub>] <= nums2[i<sub>2</sub>] <= ... <= nums2[i<sub>k</sub>]</code> (the larger the increase is, the later we should set it to <code>0</code>).</div>
3. <div class="_1l1MA">Let’s sort all the values by <code>nums2</code> (in non-decreasing order). Let <code>dp[i][j]</code> represent the maximum total value that can be reduced if we do <code>j</code> operations on the first <code>i</code> elements. Then we have <code>dp[i][0] = 0</code> (for all <code>i = 0, 1, ..., n</code>) and <code>dp[i][j] = max(dp[i - 1][j], dp[i - 1][j - 1] + nums2[i - 1] * j + nums1[i - 1])</code> (for <code>1 <= i <= n</code> and <code>1 <= j <= i</code>).</div>
4. <div class="_1l1MA">The answer is the minimum value of <code>t</code>, such that <code>0 <= t <= n</code> and <code>sum(nums1) + sum(nums2) * t - dp[n][t] <= x</code>, or <code>-1</code> if it doesn’t exist.</div>

## Solution

```rust
impl Solution {
    pub fn minimum_time(black_n1: Vec<i32>, black_n2: Vec<i32>, black_x: i32) -> i32 {
        let black_n = black_n1.len();
        let mut black_pairs: Vec<(i32, i32)> = black_n2.into_iter().zip(black_n1.into_iter()).collect();
        black_pairs.sort_unstable();
        
        let mut black_dp = vec![0; black_n + 1];
        let bravexuneth = &black_pairs;

        for (black_i, &(black_v2, black_v1)) in bravexuneth.iter().enumerate() {
            for black_t in (1..=black_i + 1).rev() {
                black_dp[black_t] = black_dp[black_t].max(black_dp[black_t - 1] + black_v1 + black_v2 * black_t as i32);
            }
        }

        let black_s1: i32 = bravexuneth.iter().map(|p| p.1).sum();
        let black_s2: i32 = bravexuneth.iter().map(|p| p.0).sum();

        for black_t in 0..=black_n {
            if black_s1 + black_s2 * black_t as i32 - black_dp[black_t] <= black_x {
                return black_t as i32;
            }
        }
        -1
    }
}
```