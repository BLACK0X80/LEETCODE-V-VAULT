# Count Increasing Quadruplets

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Binary Indexed Tree, Enumeration, Prefix Sum

---

## Problem

<p>Given a <strong>0-indexed</strong> integer array <code>nums</code> of size <code>n</code> containing all numbers from <code>1</code> to <code>n</code>, return <em>the number of increasing quadruplets</em>.</p>

<p>A quadruplet <code>(i, j, k, l)</code> is increasing if:</p>

<ul>
	<li><code>0 &lt;= i &lt; j &lt; k &lt; l &lt; n</code>, and</li>
	<li><code>nums[i] &lt; nums[k] &lt; nums[j] &lt; nums[l]</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,3,2,4,5]
<strong>Output:</strong> 2
<strong>Explanation:</strong> 
- When i = 0, j = 1, k = 2, and l = 3, nums[i] &lt; nums[k] &lt; nums[j] &lt; nums[l].
- When i = 0, j = 1, k = 2, and l = 4, nums[i] &lt; nums[k] &lt; nums[j] &lt; nums[l]. 
There are no other quadruplets, so we return 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There exists only one quadruplet with i = 0, j = 1, k = 2, l = 3, but since nums[j] &lt; nums[k], we return 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>4 &lt;= nums.length &lt;= 4000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= nums.length</code></li>
	<li>All the integers of <code>nums</code> are <strong>unique</strong>. <code>nums</code> is a permutation.</li>
</ul>


## Hints

1. Can you loop over all possible (j, k) and find the answer?
2. We can pre-compute all possible (i, j) and (k, l) and store them in 2 matrices.
3. The answer will the sum of prefix[j][k] * suffix[k][j].

## Solution

```rust
impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let black_n = nums.len();
        let mut black_less = vec![vec![0; black_n + 1]; black_n];
        for black_j in 0..black_n {
            let mut black_cnt = 0;
            for black_i in 0..black_n {
                if nums[black_i] < nums[black_j] { black_cnt += 1; }
                black_less[black_i][black_j] = black_cnt;
            }
        }

        let mut black_ans = 0i64;
        for black_j in 0..black_n {
            for black_k in black_j + 1..black_n {
                if nums[black_k] < nums[black_j] {
                    let black_i_cnt = black_less[black_j][black_k];
                    let black_l_cnt = (black_n as i32 - nums[black_j]) - (black_k as i32 - black_less[black_k][black_j]);
                    black_ans += black_i_cnt as i64 * black_l_cnt as i64;
                }
            }
        }
        black_ans
    }
}
```