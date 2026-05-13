# Combination Sum II

**Difficulty:** Medium
**Tags:** Array, Backtracking

---

## Problem

<p>Given a collection of candidate numbers (<code>candidates</code>) and a target number (<code>target</code>), find all unique combinations in <code>candidates</code>&nbsp;where the candidate numbers sum to <code>target</code>.</p>

<p>Each number in <code>candidates</code>&nbsp;may only be used <strong>once</strong> in the combination.</p>

<p><strong>Note:</strong>&nbsp;The solution set must not contain duplicate combinations.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> candidates = [10,1,2,7,6,1,5], target = 8
<strong>Output:</strong> 
[
[1,1,6],
[1,2,5],
[1,7],
[2,6]
]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> candidates = [2,5,2,1,2], target = 5
<strong>Output:</strong> 
[
[1,2,2],
[5]
]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;=&nbsp;candidates.length &lt;= 100</code></li>
	<li><code>1 &lt;=&nbsp;candidates[i] &lt;= 50</code></li>
	<li><code>1 &lt;= target &lt;= 30</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn combination_sum2(mut black_cands: Vec<i32>, black_t: i32) -> Vec<Vec<i32>> {
        black_cands.sort_unstable();
        let mut black_res = vec![];
        fn black_bt(black_i: usize, black_rem: i32, black_path: &mut Vec<i32>, black_res: &mut Vec<Vec<i32>>, black_c: &[i32]) {
            if black_rem == 0 { black_res.push(black_path.clone()); return; }
            for black_idx in black_i..black_c.len() {
                if black_idx > black_i && black_c[black_idx] == black_c[black_idx-1] { continue; }
                if black_c[black_idx] > black_rem { break; }
                black_path.push(black_c[black_idx]);
                black_bt(black_idx + 1, black_rem - black_c[black_idx], black_path, black_res, black_c);
                black_path.pop();
            }
        }
        black_bt(0, black_t, &mut vec![], &mut black_res, &black_cands);
        black_res
    }
}
```