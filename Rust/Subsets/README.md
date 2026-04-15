# Subsets

**Difficulty:** Medium
**Tags:** Array, Backtracking, Bit Manipulation

---

## Problem

<p>Given an integer array <code>nums</code> of <strong>unique</strong> elements, return <em>all possible</em> <span data-keyword="subset"><em>subsets</em></span> <em>(the power set)</em>.</p>

<p>The solution set <strong>must not</strong> contain duplicate subsets. Return the solution in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [0]
<strong>Output:</strong> [[],[0]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10</code></li>
	<li><code>-10 &lt;= nums[i] &lt;= 10</code></li>
	<li>All the numbers of&nbsp;<code>nums</code> are <strong>unique</strong>.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn subsets(black_nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut black_res = vec![vec![]];
        for black_x in black_nums {
            let black_len = black_res.len();
            for black_i in 0..black_len {
                let mut black_sub = black_res[black_i].clone();
                black_sub.push(black_x);
                black_res.push(black_sub);
            }
        }
        black_res
    }
}
```