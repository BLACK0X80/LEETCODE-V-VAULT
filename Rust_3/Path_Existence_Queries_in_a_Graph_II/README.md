# Path Existence Queries in a Graph II

**Difficulty:** Hard
**Tags:** Array, Two Pointers, Binary Search, Dynamic Programming, Greedy, Bit Manipulation, Graph Theory, Sorting

---

## Problem

<p>You are given an integer <code>n</code> representing the number of nodes in a graph, labeled from 0 to <code>n - 1</code>.</p>

<p>You are also given an integer array <code>nums</code> of length <code>n</code> and an integer <code>maxDiff</code>.</p>

<p>An <strong>undirected </strong>edge exists between nodes <code>i</code> and <code>j</code> if the <strong>absolute</strong> difference between <code>nums[i]</code> and <code>nums[j]</code> is <strong>at most</strong> <code>maxDiff</code> (i.e., <code>|nums[i] - nums[j]| &lt;= maxDiff</code>).</p>

<p>You are also given a 2D integer array <code>queries</code>. For each <code>queries[i] = [u<sub>i</sub>, v<sub>i</sub>]</code>, find the <strong>minimum</strong> distance between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code><sub>.</sub> If no path exists between the two nodes, return -1 for that query.</p>

<p>Return an array <code>answer</code>, where <code>answer[i]</code> is the result of the <code>i<sup>th</sup></code> query.</p>

<p><strong>Note:</strong> The edges between the nodes are unweighted.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, nums = [1,8,3,4,2], maxDiff = 3, queries = [[0,3],[2,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,1]</span></p>

<p><strong>Explanation:</strong></p>

<p>The resulting graph is:</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/03/25/4149example1drawio.png" style="width: 281px; height: 161px;" /></p>

<table>
	<tbody>
		<tr>
			<th>Query</th>
			<th>Shortest Path</th>
			<th>Minimum Distance</th>
		</tr>
		<tr>
			<td>[0, 3]</td>
			<td>0 &rarr; 3</td>
			<td>1</td>
		</tr>
		<tr>
			<td>[2, 4]</td>
			<td>2 &rarr; 4</td>
			<td>1</td>
		</tr>
	</tbody>
</table>

<p>Thus, the output is <code>[1, 1]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, nums = [5,3,1,9,10], maxDiff = 2, queries = [[0,1],[0,2],[2,3],[4,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,2,-1,1]</span></p>

<p><strong>Explanation:</strong></p>

<p>The resulting graph is:</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/03/25/4149example2drawio.png" style="width: 281px; height: 121px;" /></p>
</div>

<table>
	<tbody>
		<tr>
			<th>Query</th>
			<th>Shortest Path</th>
			<th>Minimum Distance</th>
		</tr>
		<tr>
			<td>[0, 1]</td>
			<td>0 &rarr; 1</td>
			<td>1</td>
		</tr>
		<tr>
			<td>[0, 2]</td>
			<td>0 &rarr; 1 &rarr; 2</td>
			<td>2</td>
		</tr>
		<tr>
			<td>[2, 3]</td>
			<td>None</td>
			<td>-1</td>
		</tr>
		<tr>
			<td>[4, 3]</td>
			<td>3 &rarr; 4</td>
			<td>1</td>
		</tr>
	</tbody>
</table>

<p>Thus, the output is <code>[1, 2, -1, 1]</code>.</p>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, nums = [3,6,1], maxDiff = 1, queries = [[0,0],[0,1],[1,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,-1,-1]</span></p>

<p><strong>Explanation:</strong></p>

<p>There are no edges between any two nodes because:</p>

<ul>
	<li>Nodes 0 and 1: <code>|nums[0] - nums[1]| = |3 - 6| = 3 &gt; 1</code></li>
	<li>Nodes 0 and 2: <code>|nums[0] - nums[2]| = |3 - 1| = 2 &gt; 1</code></li>
	<li>Nodes 1 and 2: <code>|nums[1] - nums[2]| = |6 - 1| = 5 &gt; 1</code></li>
</ul>

<p>Thus, no node can reach any other node, and the output is <code>[0, -1, -1]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= maxDiff &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>queries[i] == [u<sub>i</sub>, v<sub>i</sub>]</code></li>
	<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt; n</code></li>
</ul>


## Hints

1. Sort the nodes according to <code>nums[i]</code>.
2. Can we use binary jumping?
3. Use binary jumping with a sparse table data structure.

## Solution

```rust
impl Solution {
    pub fn path_existence_queries(black1: i32, black2: Vec<i32>, black3: i32, black4: Vec<Vec<i32>>) -> Vec<i32> {
        let mut black5 = black2.clone();
        black5.sort_unstable();
        black5.dedup();
        let black6 = black5.len();
        
        let mut black7 = vec![0; black6];
        for i in 1..black6 {
            black7[i] = black7[i - 1] + (black5[i] - black5[i - 1] > black3) as i32;
        }
        
        let mut black8 = vec![vec![0; 18]; black6];
        let mut r = 0;
        for i in 0..black6 {
            while r < black6 && black5[r] <= black5[i] + black3 { r += 1; }
            black8[i][0] = r - 1;
        }
        
        for j in 1..18 {
            for i in 0..black6 {
                black8[i][j] = black8[black8[i][j - 1]][j - 1];
            }
        }

        black4.into_iter().map(|q| {
            let (b_u, b_v) = (q[0] as usize, q[1] as usize);
            if b_u == b_v { return 0; }
            
            let (u_val, v_val) = (black2[b_u], black2[b_v]);
            let (mut x, mut y) = (black5.binary_search(&u_val).unwrap(), black5.binary_search(&v_val).unwrap());
            if x > y { std::mem::swap(&mut x, &mut y); }
            
            if black7[x] != black7[y] { return -1; }
            
            let (mut res, mut curr) = (0, x);
            for j in (0..18).rev() {
                if black8[curr][j] < y {
                    curr = black8[curr][j];
                    res |= 1 << j;
                }
            }
            res + 1
        }).collect()
    }
}
```