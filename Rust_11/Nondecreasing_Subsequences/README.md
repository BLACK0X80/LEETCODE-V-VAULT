# Non-decreasing Subsequences

**Difficulty:** Medium
**Tags:** Array, Hash Table, Backtracking, Bit Manipulation

---

## Problem

<p>Given an integer array <code>nums</code>, return <em>all the different possible non-decreasing subsequences of the given array with at least two elements</em>. You may return the answer in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [4,6,7,7]
<strong>Output:</strong> [[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [4,4,3,2,1]
<strong>Output:</strong> [[4,4]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 15</code></li>
	<li><code>-100 &lt;= nums[i] &lt;= 100</code></li>
</ul>



## Solution

```rust
impl Solution { pub fn find_subsequences(black_n: Vec<i32>) -> Vec<Vec<i32>> { let mut black_res = std::collections::HashSet::new(); fn black_bt(idx: usize, cur: &mut Vec<i32>, n: &[i32], res: &mut std::collections::HashSet<Vec<i32>>) { if cur.len() >= 2 { res.insert(cur.clone()); } for i in idx..n.len() { if cur.is_empty() || n[i] >= *cur.last().unwrap() { cur.push(n[i]); black_bt(i + 1, cur, n, res); cur.pop(); } } } black_bt(0, &mut vec![], &black_n, &mut black_res); black_res.into_iter().collect() } }
```