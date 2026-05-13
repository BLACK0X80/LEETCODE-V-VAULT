# Properties Graph

**Difficulty:** Medium
**Tags:** Array, Hash Table, Depth-First Search, Breadth-First Search, Union-Find, Graph Theory

---

## Problem

<p>You are given a 2D integer array <code>properties</code> having dimensions <code>n x m</code> and an integer <code>k</code>.</p>

<p>Define a function <code>intersect(a, b)</code> that returns the <strong>number of distinct integers</strong> common to both arrays <code>a</code> and <code>b</code>.</p>

<p>Construct an <strong>undirected</strong> graph where each index <code>i</code> corresponds to <code>properties[i]</code>. There is an edge between node <code>i</code> and node <code>j</code> if and only if <code>intersect(properties[i], properties[j]) &gt;= k</code>, where <code>i</code> and <code>j</code> are in the range <code>[0, n - 1]</code> and <code>i != j</code>.</p>

<p>Return the number of <strong>connected components</strong> in the resulting graph.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">properties = [[1,2],[1,1],[3,4],[4,5],[5,6],[7,7]], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The graph formed has 3 connected components:</p>

<p><img height="171" src="https://assets.leetcode.com/uploads/2025/02/27/image.png" width="279" /></p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">properties = [[1,2,3],[2,3,4],[4,3,5]], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The graph formed has 1 connected component:</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/02/27/screenshot-from-2025-02-27-23-58-34.png" style="width: 219px; height: 171px;" /></p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">properties = [[1,1],[1,1]], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p><code>intersect(properties[0], properties[1]) = 1</code>, which is less than <code>k</code>. This means there is no edge between <code>properties[0]</code> and <code>properties[1]</code> in the graph.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == properties.length &lt;= 100</code></li>
	<li><code>1 &lt;= m == properties[i].length &lt;= 100</code></li>
	<li><code>1 &lt;= properties[i][j] &lt;= 100</code></li>
	<li><code>1 &lt;= k &lt;= m</code></li>
</ul>


## Hints

1. How can we optimally find the intersection of two arrays? One way is to use <code>len(set(a) & set(b))</code>.
2. For connected components, think about using DFS, BFS, or DSU.

## Solution

```rust
impl Solution { pub fn number_of_components(black_p: Vec<Vec<i32>>, black_k: i32) -> i32 { let black_n = black_p.len(); let mut black_ds = vec![-1; black_n]; let mut black_ps: Vec<Vec<i32>> = black_p.into_iter().map(|mut black_v| { black_v.sort(); black_v.dedup(); black_v }).collect(); fn black_f(black_i: usize, black_ds: &mut Vec<i32>) -> usize { if black_ds[black_i] < 0 { black_i } else { black_ds[black_i] = black_f(black_ds[black_i] as usize, black_ds) as i32; black_ds[black_i] as usize } } for black_i in 0..black_n { for black_j in black_i + 1..black_n { let (black_a, black_b) = (black_f(black_i, &mut black_ds), black_f(black_j, &mut black_ds)); if black_a != black_b { let (mut black_c, mut black_p1, mut black_p2) = (0, 0, 0); let (black_s1, black_s2) = (&black_ps[black_i], &black_ps[black_j]); while black_p1 < black_s1.len() && black_p2 < black_s2.len() { if black_s1[black_p1] == black_s2[black_p2] { black_c += 1; black_p1 += 1; black_p2 += 1; } else if black_s1[black_p1] < black_s2[black_p2] { black_p1 += 1; } else { black_p2 += 1; } if black_c >= black_k { break; } } if black_c >= black_k { black_ds[black_b] = black_a as i32; } } } } black_ds.iter().filter(|&&black_x| black_x < 0).count() as i32 } }
```