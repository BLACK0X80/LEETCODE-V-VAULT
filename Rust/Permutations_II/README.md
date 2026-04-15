# Permutations II

**Difficulty:** Medium
**Tags:** Array, Backtracking, Sorting

---

## Problem

<p>Given a collection of numbers, <code>nums</code>,&nbsp;that might contain duplicates, return <em>all possible unique permutations <strong>in any order</strong>.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,2]
<strong>Output:</strong>
[[1,1,2],
 [1,2,1],
 [2,1,1]]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 8</code></li>
	<li><code>-10 &lt;= nums[i] &lt;= 10</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn permute_unique(mut black_nums: Vec<i32>) -> Vec<Vec<i32>> {
        black_nums.sort_unstable();
        let mut black_res = vec![];
        fn black_bt(mut black_nums: Vec<i32>, black_start: usize, black_res: &mut Vec<Vec<i32>>) {
            if black_start == black_nums.len() { black_res.push(black_nums); return; }
            for black_i in black_start..black_nums.len() {
                if black_i > black_start && (black_start..black_i).any(|idx| black_nums[idx] == black_nums[black_i]) { continue; }
                black_nums.swap(black_start, black_i);
                black_bt(black_nums.clone(), black_start + 1, black_res);
            }
        }
        black_bt(black_nums, 0, &mut black_res);
        black_res
    }
}
```