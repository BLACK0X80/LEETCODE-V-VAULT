# Count Connected Components in LCM Graph

**Difficulty:** Hard
**Tags:** Array, Hash Table, Math, Union-Find, Number Theory

---

## Problem

<p>You are given an array of integers <code>nums</code> of size <code>n</code> and a <strong>positive</strong> integer <code>threshold</code>.</p>

<p>There is a graph consisting of <code>n</code> nodes with the&nbsp;<code>i<sup>th</sup></code>&nbsp;node having a value of <code>nums[i]</code>. Two nodes <code>i</code> and <code>j</code> in the graph are connected via an <strong>undirected</strong> edge if <code>lcm(nums[i], nums[j]) &lt;= threshold</code>.</p>

<p>Return the number of <strong>connected components</strong> in this graph.</p>

<p>A <strong>connected component</strong> is a subgraph of a graph in which there exists a path between any two vertices, and no vertex of the subgraph shares an edge with a vertex outside of the subgraph.</p>

<p>The term <code>lcm(a, b)</code> denotes the <strong>least common multiple</strong> of <code>a</code> and <code>b</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,4,8,3,9], threshold = 5</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong>&nbsp;</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/10/31/example0.png" style="width: 250px; height: 251px;" /></p>

<p>&nbsp;</p>

<p>The four connected components are <code>(2, 4)</code>, <code>(3)</code>, <code>(8)</code>, <code>(9)</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,4,8,3,9,12], threshold = 10</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong>&nbsp;</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/10/31/example1.png" style="width: 250px; height: 252px;" /></p>

<p>The two connected components are <code>(2, 3, 4, 8, 9)</code>, and <code>(12)</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li>All elements of <code>nums</code> are unique.</li>
	<li><code>1 &lt;= threshold &lt;= 2 * 10<sup>5</sup></code></li>
</ul>


## Hints

1. Use DSU
2. Connect a number to all its multiples less than threshold

## Solution

```rust
impl Solution {
    pub fn count_components(black_nums: Vec<i32>, black_threshold: i32) -> i32 {
        let black_threshold = black_threshold as usize;
        let mut black_parent: Vec<usize> = (0..=black_threshold).collect();
        let mut black_exists = vec![false; black_threshold + 1];
        let mut black_large_count = 0;

        fn black_find(black_i: usize, black_p: &mut Vec<usize>) -> usize {
            if black_p[black_i] == black_i { black_i }
            else { black_p[black_i] = black_find(black_p[black_i], black_p); black_p[black_i] }
        }

        fn black_union(black_i: usize, black_j: usize, black_p: &mut Vec<usize>) {
            let black_root_i = black_find(black_i, black_p);
            let black_root_j = black_find(black_j, black_p);
            if black_root_i != black_root_j { black_p[black_root_i] = black_root_j; }
        }

        for &black_x in &black_nums {
            if black_x as usize <= black_threshold {
                black_exists[black_x as usize] = true;
            } else {
                black_large_count += 1;
            }
        }

        for black_g in 1..=black_threshold {
            let mut black_first = 0;
            for black_v in (black_g..=black_threshold).step_by(black_g) {
                if black_exists[black_v] {
                    if black_first == 0 {
                        black_first = black_v;
                    } else {
                        if (black_first as u64 * black_v as u64) / black_g as u64 <= black_threshold as u64 {
                            black_union(black_first, black_v, &mut black_parent);
                        }
                    }
                }
            }
        }

        let mut black_roots = std::collections::HashSet::new();
        for black_i in 1..=black_threshold {
            if black_exists[black_i] {
                black_roots.insert(black_find(black_i, &mut black_parent));
            }
        }

        (black_roots.len() + black_large_count) as i32
    }
}
```