# Combination Sum

**Difficulty:** Medium
**Tags:** Array, Backtracking

---

## Problem

<p>Given an array of <strong>distinct</strong> integers <code>candidates</code> and a target integer <code>target</code>, return <em>a list of all <strong>unique combinations</strong> of </em><code>candidates</code><em> where the chosen numbers sum to </em><code>target</code><em>.</em> You may return the combinations in <strong>any order</strong>.</p>

<p>The <strong>same</strong> number may be chosen from <code>candidates</code> an <strong>unlimited number of times</strong>. Two combinations are unique if the <span data-keyword="frequency-array">frequency</span> of at least one of the chosen numbers is different.</p>

<p>The test cases are generated such that the number of unique combinations that sum up to <code>target</code> is less than <code>150</code> combinations for the given input.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> candidates = [2,3,6,7], target = 7
<strong>Output:</strong> [[2,2,3],[7]]
<strong>Explanation:</strong>
2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
7 is a candidate, and 7 = 7.
These are the only two combinations.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> candidates = [2,3,5], target = 8
<strong>Output:</strong> [[2,2,2,2],[2,3,3],[3,5]]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> candidates = [2], target = 1
<strong>Output:</strong> []
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= candidates.length &lt;= 30</code></li>
	<li><code>2 &lt;= candidates[i] &lt;= 40</code></li>
	<li>All elements of <code>candidates</code> are <strong>distinct</strong>.</li>
	<li><code>1 &lt;= target &lt;= 40</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn combination_sum(black_cands: Vec<i32>, black_t: i32) -> Vec<Vec<i32>> {
        let mut black_res = vec![];
        fn black_bt(black_i: usize, black_rem: i32, black_path: &mut Vec<i32>, black_res: &mut Vec<Vec<i32>>, black_c: &[i32]) {
            if black_rem == 0 { black_res.push(black_path.clone()); return; }
            for black_idx in black_i..black_c.len() {
                if black_c[black_idx] <= black_rem {
                    black_path.push(black_c[black_idx]);
                    black_bt(black_idx, black_rem - black_c[black_idx], black_path, black_res, black_c);
                    black_path.pop();
                }
            }
        }
        black_bt(0, black_t, &mut vec![], &mut black_res, &black_cands);
        black_res
    }
}
```