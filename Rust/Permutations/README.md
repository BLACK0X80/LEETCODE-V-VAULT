# Permutations

**Difficulty:** Medium
**Tags:** Array, Backtracking

---

## Problem

<p>Given an array <code>nums</code> of distinct integers, return all the possible <span data-keyword="permutation-array">permutations</span>. You can return the answer in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> nums = [0,1]
<strong>Output:</strong> [[0,1],[1,0]]
</pre><p><strong class="example">Example 3:</strong></p>
<pre><strong>Input:</strong> nums = [1]
<strong>Output:</strong> [[1]]
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 6</code></li>
	<li><code>-10 &lt;= nums[i] &lt;= 10</code></li>
	<li>All the integers of <code>nums</code> are <strong>unique</strong>.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn permute(mut black_nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut black_res = vec![];
        fn black_bt(black_start: usize, black_nums: &mut Vec<i32>, black_res: &mut Vec<Vec<i32>>) {
            if black_start == black_nums.len() { black_res.push(black_nums.clone()); return; }
            for black_i in black_start..black_nums.len() {
                black_nums.swap(black_start, black_i);
                black_bt(black_start + 1, black_nums, black_res);
                black_nums.swap(black_start, black_i);
            }
        }
        black_bt(0, &mut black_nums, &mut black_res);
        black_res
    }
}
```