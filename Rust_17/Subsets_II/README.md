# Subsets II

**Difficulty:** Medium
**Tags:** Array, Backtracking, Bit Manipulation

---

## Problem

<p>Given an integer array <code>nums</code> that may contain duplicates, return <em>all possible</em> <span data-keyword="subset"><em>subsets</em></span><em> (the power set)</em>.</p>

<p>The solution set <strong>must not</strong> contain duplicate subsets. Return the solution in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> nums = [1,2,2]
<strong>Output:</strong> [[],[1],[1,2],[1,2,2],[2],[2,2]]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> nums = [0]
<strong>Output:</strong> [[],[0]]
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10</code></li>
	<li><code>-10 &lt;= nums[i] &lt;= 10</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn subsets_with_dup(mut black_nums: Vec<i32>) -> Vec<Vec<i32>> {
        black_nums.sort_unstable();
        let mut black_res = vec![vec![]];
        let (mut black_start, mut black_end) = (0, 0);
        for black_i in 0..black_nums.len() {
            black_start = if black_i > 0 && black_nums[black_i] == black_nums[black_i-1] { black_end } else { 0 };
            black_end = black_res.len();
            for black_j in black_start..black_end {
                let mut black_subset = black_res[black_j].clone();
                black_subset.push(black_nums[black_i]);
                black_res.push(black_subset);
            }
        }
        black_res
    }
}
```