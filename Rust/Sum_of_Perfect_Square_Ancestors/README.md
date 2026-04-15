# Sum of Perfect Square Ancestors

**Difficulty:** Hard
**Tags:** Array, Hash Table, Math, Tree, Depth-First Search, Counting, Number Theory

---

## Problem

<p>You are given an integer <code>n</code> and an undirected tree rooted at node 0 with <code>n</code> nodes numbered from 0 to <code>n - 1</code>. This is represented by a 2D array <code>edges</code> of length <code>n - 1</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> indicates an undirected edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>.</p>

<p>You are also given an integer array <code>nums</code>, where <code>nums[i]</code> is the positive integer assigned to node <code>i</code>.</p>

<p>Define a value <code>t<sub>i</sub></code> as the number of <strong>ancestors</strong> of node <code>i</code> such that the product <code>nums[i] * nums[ancestor]</code> is a <strong><span data-keyword="perfect-square">perfect square</span></strong>.</p>

<p>Return the sum of all <code>t<sub>i</sub></code> values for all nodes <code>i</code> in range <code>[1, n - 1]</code>.</p>

<p><strong>Note</strong>:</p>

<ul>
	<li>In a rooted tree, the <strong>ancestors</strong> of node <code>i</code> are all nodes on the path from node <code>i</code> to the root node 0, <strong>excluding</strong> <code>i</code> itself.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, edges = [[0,1],[1,2]], nums = [2,8,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;"><code><strong>i</strong></code></th>
			<th style="border: 1px solid black;"><strong>Ancestors</strong></th>
			<th style="border: 1px solid black;"><code><strong>nums[i] * nums[ancestor]</strong></code></th>
			<th style="border: 1px solid black;">Square Check</th>
			<th style="border: 1px solid black;"><code><strong>t<sub>i</sub></strong></code></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">[0]</td>
			<td style="border: 1px solid black;"><code>nums[1] * nums[0] = 8 * 2 = 16</code></td>
			<td style="border: 1px solid black;">16 is a perfect square</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">[1, 0]</td>
			<td style="border: 1px solid black;"><code>nums[2] * nums[1] = 2 * 8 = 16</code><br />
			<code>nums[2] * nums[0] = 2 * 2 = 4</code></td>
			<td style="border: 1px solid black;">Both 4 and 16 are perfect squares</td>
			<td style="border: 1px solid black;">2</td>
		</tr>
	</tbody>
</table>

<p>Thus, the total number of valid ancestor pairs across all non-root nodes is <code>1 + 2 = 3</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, edges = [[0,1],[0,2]], nums = [1,2,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;"><code><strong>i</strong></code></th>
			<th style="border: 1px solid black;"><strong>Ancestors</strong></th>
			<th style="border: 1px solid black;"><code><strong>nums[i] * nums[ancestor]</strong></code></th>
			<th style="border: 1px solid black;">Square Check</th>
			<th style="border: 1px solid black;"><code><strong>t<sub>i</sub></strong></code></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">[0]</td>
			<td style="border: 1px solid black;"><code>nums[1] * nums[0] = 2 * 1 = 2</code></td>
			<td style="border: 1px solid black;">2 is <strong>not</strong> a perfect square</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">[0]</td>
			<td style="border: 1px solid black;"><code>nums[2] * nums[0] = 4 * 1 = 4</code></td>
			<td style="border: 1px solid black;">4 is a perfect square</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
	</tbody>
</table>

<p data-end="996" data-start="929">Thus, the total number of valid ancestor pairs across all non-root nodes is 1.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4, edges = [[0,1],[0,2],[1,3]], nums = [1,2,9,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;"><code>i</code></th>
			<th style="border: 1px solid black;"><strong>Ancestors</strong></th>
			<th style="border: 1px solid black;"><code><strong>nums[i] * nums[ancestor]</strong></code></th>
			<th style="border: 1px solid black;">Square Check</th>
			<th style="border: 1px solid black;"><code><strong>t<sub>i</sub></strong></code></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">[0]</td>
			<td style="border: 1px solid black;"><code>nums[1] * nums[0] = 2 * 1 = 2</code></td>
			<td style="border: 1px solid black;">2 is <strong>not</strong> a perfect square</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">[0]</td>
			<td style="border: 1px solid black;"><code>nums[2] * nums[0] = 9 * 1 = 9</code></td>
			<td style="border: 1px solid black;">9 is a perfect square</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">[1, 0]</td>
			<td style="border: 1px solid black;"><code>nums[3] * nums[1] = 4 * 2 = 8</code><br />
			<code>nums[3] * nums[0] = 4 * 1 = 4</code></td>
			<td style="border: 1px solid black;">Only 4 is a perfect square</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
	</tbody>
</table>

<p>Thus, the total number of valid ancestor pairs across all non-root nodes is <code>0 + 1 + 1 = 2</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code></li>
	<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt;= n - 1</code></li>
	<li><code>nums.length == n</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li>The input is generated such that <code>edges</code> represents a valid tree.</li>
</ul>


## Hints

1. Notice that the product <code>nums[i] * nums[ancestor]</code> is a perfect square if and only if both numbers have the same "square-free kernel" (i.e., after removing all even powers of primes, the remaining product is identical).
2. Precompute the square-free representation of every node's value using prime factorization up to <code>max(nums[i])</code>.
3. Perform a DFS from the root. While traversing down the tree, maintain a frequency map of the square-free values of the ancestors.
4. For each node, the number of valid ancestors equals the count of ancestors with the same square-free value.
5. Carefully backtrack the frequency map after finishing a subtree to maintain correctness.

## Solution

```rust
impl Solution {
    pub fn sum_of_ancestors(n: i32, edges: Vec<Vec<i32>>, nums: Vec<i32>) -> i64 {
        let mut black_adj = vec![vec![]; n as usize];
        for e in edges {
            black_adj[e[0] as usize].push(e[1] as usize);
            black_adj[e[1] as usize].push(e[0] as usize);
        }

        let mut black_core = vec![0; n as usize];
        for i in 0..n as usize {
            let (mut v, mut c, mut d) = (nums[i], 1, 2);
            while d * d <= v {
                let mut count = 0;
                while v % d == 0 { v /= d; count += 1; }
                if count % 2 != 0 { c *= d; }
                d += 1;
            }
            if v > 1 { c *= v; }
            black_core[i] = c;
        }

        let mut black_ans = 0i64;
        let mut black_counts = std::collections::HashMap::new();

        fn dfs(u: usize, p: usize, adj: &Vec<Vec<usize>>, core: &Vec<i32>, counts: &mut std::collections::HashMap<i32, i32>, ans: &mut i64) {
            let c = core[u];
            if u != 0 { *ans += *counts.get(&c).unwrap_or(&0) as i64; }
            *counts.entry(c).or_insert(0) += 1;
            for &v in &adj[u] {
                if v != p { dfs(v, u, adj, core, counts, ans); }
            }
            *counts.get_mut(&c).unwrap() -= 1;
        }

        dfs(0, 0, &black_adj, &black_core, &mut black_counts, &mut black_ans);
        black_ans
    }
}
```