# Sort Items by Groups Respecting Dependencies

**Difficulty:** Hard
**Tags:** Depth-First Search, Breadth-First Search, Graph Theory, Topological Sort

---

## Problem

<p>There are&nbsp;<code>n</code>&nbsp;items each&nbsp;belonging to zero or one of&nbsp;<code>m</code>&nbsp;groups where <code>group[i]</code>&nbsp;is the group that the <code>i</code>-th item belongs to and it&#39;s equal to <code>-1</code>&nbsp;if the <code>i</code>-th item belongs to no group. The items and the groups are zero indexed. A group can have no item belonging to it.</p>

<p>Return a sorted list of the items such that:</p>

<ul>
	<li>The items that belong to the same group are next to each other in the sorted list.</li>
	<li>There are some&nbsp;relations&nbsp;between these items where&nbsp;<code>beforeItems[i]</code>&nbsp;is a list containing all the items that should come before the&nbsp;<code>i</code>-th item in the sorted array (to the left of the&nbsp;<code>i</code>-th item).</li>
</ul>

<p>Return any solution if there is more than one solution and return an <strong>empty list</strong>&nbsp;if there is no solution.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2019/09/11/1359_ex1.png" style="width: 191px; height: 181px;" /></strong></p>

<pre>
<strong>Input:</strong> n = 8, m = 2, group = [-1,-1,1,0,0,1,0,-1], beforeItems = [[],[6],[5],[6],[3,6],[],[],[]]
<strong>Output:</strong> [6,3,4,1,5,2,0,7]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 8, m = 2, group = [-1,-1,1,0,0,1,0,-1], beforeItems = [[],[6],[5],[6],[3],[],[4],[]]
<strong>Output:</strong> []
<strong>Explanation:</strong>&nbsp;This is the same as example 1 except that 4 needs to be before 6 in the sorted list.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m &lt;= n &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>group.length == beforeItems.length == n</code></li>
	<li><code>-1 &lt;= group[i] &lt;= m - 1</code></li>
	<li><code>0 &lt;= beforeItems[i].length &lt;= n - 1</code></li>
	<li><code>0 &lt;= beforeItems[i][j] &lt;= n - 1</code></li>
	<li><code>i != beforeItems[i][j]</code></li>
	<li><code>beforeItems[i]&nbsp;</code>does not contain&nbsp;duplicates elements.</li>
</ul>


## Hints

1. Think of it as a graph problem.
2. We need to find a topological order on the dependency graph.
3. Build two graphs, one for the groups and another for the items.

## Solution

```rust
use std::collections::{VecDeque, HashMap};

impl Solution {
    pub fn sort_items(black1: i32, black2: i32, mut black3: Vec<i32>, black4: Vec<Vec<i32>>) -> Vec<i32> {
        let black5 = black1 as usize;
        let mut black6 = black2;
        
        for black7 in 0..black5 {
            if black3[black7] == -1 {
                black3[black7] = black6;
                black6 += 1;
            }
        }
        let black8 = black6 as usize;

        fn black_topo(black9: Vec<usize>, black10: &Vec<Vec<usize>>) -> Vec<usize> {
            let mut black11 = HashMap::new();
            for &black12 in &black9 {
                black11.insert(black12, 0);
            }
            for &black12 in &black9 {
                for &black13 in &black10[black12] {
                    if black11.contains_key(&black13) {
                        *black11.get_mut(&black13).unwrap() += 1;
                    }
                }
            }
            
            let mut black14 = VecDeque::new();
            for (&black15, &black16) in &black11 {
                if black16 == 0 {
                    black14.push_back(black15);
                }
            }
            
            let mut black17 = Vec::new();
            while let Some(black18) = black14.pop_front() {
                black17.push(black18);
                for &black19 in &black10[black18] {
                    if let Some(black20) = black11.get_mut(&black19) {
                        *black20 -= 1;
                        if *black20 == 0 {
                            black14.push_back(black19);
                        }
                    }
                }
            }
            if black17.len() == black9.len() { black17 } else { vec![] }
        }

        let mut black21 = vec![vec![]; black5];
        let mut black22 = vec![vec![]; black8];
        for black23 in 0..black5 {
            for &black24 in &black4[black23] {
                let black25 = black24 as usize;
                if black3[black25] == black3[black23] {
                    black21[black25].push(black23);
                } else {
                    black22[black3[black25] as usize].push(black3[black23] as usize);
                }
            }
        }

        let black26: Vec<usize> = (0..black8).collect();
        let black27 = black_topo(black26, &black22);
        if black27.is_empty() { return vec![]; }

        let mut black28 = vec![vec![]; black8];
        for black29 in 0..black5 {
            black28[black3[black29] as usize].push(black29);
        }

        let mut black30 = Vec::new();
        for black31 in black27 {
            let black32 = black_topo(black28[black31].clone(), &black21);
            if black32.is_empty() && !black28[black31].is_empty() { return vec![]; }
            for black33 in black32 {
                black30.push(black33 as i32);
            }
        }

        black30
    }
}
```