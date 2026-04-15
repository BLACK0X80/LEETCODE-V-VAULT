# Find Weighted Median Node in Tree

**Difficulty:** Hard
**Tags:** Array, Binary Search, Dynamic Programming, Bit Manipulation, Tree, Depth-First Search

---

## Problem

<p>You are given an integer <code>n</code> and an <strong>undirected, weighted</strong> tree rooted at node 0 with <code>n</code> nodes numbered from 0 to <code>n - 1</code>. This is represented by a 2D array <code>edges</code> of length <code>n - 1</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, w<sub>i</sub>]</code> indicates an edge from node <code>u<sub>i</sub></code> to <code>v<sub>i</sub></code> with weight <code>w<sub>i</sub></code>.</p>

<p>The <strong>weighted median node</strong> is defined as the <strong>first</strong> node <code>x</code> on the path from <code>u<sub>i</sub></code> to <code>v<sub>i</sub></code> such that the sum of edge weights from <code>u<sub>i</sub></code> to <code>x</code> is <strong>greater than or equal to half</strong> of the total path weight.</p>

<p>You are given a 2D integer array <code>queries</code>. For each <code>queries[j] = [u<sub>j</sub>, v<sub>j</sub>]</code>, determine the weighted median node along the path from <code>u<sub>j</sub></code> to <code>v<sub>j</sub></code>.</p>

<p>Return an array <code>ans</code>, where <code>ans[j]</code> is the node index of the weighted median for <code>queries[j]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 2, edges = [[0,1,7]], queries = [[1,0],[0,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,1]</span></p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/05/26/screenshot-2025-05-26-at-193447.png" style="width: 200px; height: 64px;" /></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Query</th>
			<th style="border: 1px solid black;">Path</th>
			<th style="border: 1px solid black;">Edge<br />
			Weights</th>
			<th style="border: 1px solid black;">Total<br />
			Path<br />
			Weight</th>
			<th style="border: 1px solid black;">Half</th>
			<th style="border: 1px solid black;">Explanation</th>
			<th style="border: 1px solid black;">Answer</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;"><code>[1, 0]</code></td>
			<td style="border: 1px solid black;"><code>1 &rarr; 0</code></td>
			<td style="border: 1px solid black;"><code>[7]</code></td>
			<td style="border: 1px solid black;">7</td>
			<td style="border: 1px solid black;">3.5</td>
			<td style="border: 1px solid black;">Sum from <code>1 &rarr; 0 = 7 &gt;= 3.5</code>, median is node 0.</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[0, 1]</code></td>
			<td style="border: 1px solid black;"><code>0 &rarr; 1</code></td>
			<td style="border: 1px solid black;"><code>[7]</code></td>
			<td style="border: 1px solid black;">7</td>
			<td style="border: 1px solid black;">3.5</td>
			<td style="border: 1px solid black;">Sum from <code>0 &rarr; 1 = 7 &gt;= 3.5</code>, median is node 1.</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
	</tbody>
</table>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, edges = [[0,1,2],[2,0,4]], queries = [[0,1],[2,0],[1,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,0,2]</span></p>

<p><strong>E</strong><strong>xplanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/05/26/screenshot-2025-05-26-at-193610.png" style="width: 180px; height: 149px;" /></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Query</th>
			<th style="border: 1px solid black;">Path</th>
			<th style="border: 1px solid black;">Edge<br />
			Weights</th>
			<th style="border: 1px solid black;">Total<br />
			Path<br />
			Weight</th>
			<th style="border: 1px solid black;">Half</th>
			<th style="border: 1px solid black;">Explanation</th>
			<th style="border: 1px solid black;">Answer</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;"><code>[0, 1]</code></td>
			<td style="border: 1px solid black;"><code>0 &rarr; 1</code></td>
			<td style="border: 1px solid black;"><code>[2]</code></td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">Sum from <code>0 &rarr; 1 = 2 &gt;= 1</code>, median is node 1.</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[2, 0]</code></td>
			<td style="border: 1px solid black;"><code>2 &rarr; 0</code></td>
			<td style="border: 1px solid black;"><code>[4]</code></td>
			<td style="border: 1px solid black;">4</td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">Sum from <code>2 &rarr; 0 = 4 &gt;= 2</code>, median is node 0.</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[1, 2]</code></td>
			<td style="border: 1px solid black;"><code>1 &rarr; 0 &rarr; 2</code></td>
			<td style="border: 1px solid black;"><code>[2, 4]</code></td>
			<td style="border: 1px solid black;">6</td>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;">Sum from <code>1 &rarr; 0 = 2 &lt; 3</code>.<br />
			Sum from <code>1 &rarr; 2 = 2 + 4 = 6 &gt;= 3</code>, median is node 2.</td>
			<td style="border: 1px solid black;">2</td>
		</tr>
	</tbody>
</table>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, edges = [[0,1,2],[0,2,5],[1,3,1],[2,4,3]], queries = [[3,4],[1,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2,2]</span></p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/05/26/screenshot-2025-05-26-at-193857.png" style="width: 150px; height: 229px;" /></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Query</th>
			<th style="border: 1px solid black;">Path</th>
			<th style="border: 1px solid black;">Edge<br />
			Weights</th>
			<th style="border: 1px solid black;">Total<br />
			Path<br />
			Weight</th>
			<th style="border: 1px solid black;">Half</th>
			<th style="border: 1px solid black;">Explanation</th>
			<th style="border: 1px solid black;">Answer</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;"><code>[3, 4]</code></td>
			<td style="border: 1px solid black;"><code>3 &rarr; 1 &rarr; 0 &rarr; 2 &rarr; 4</code></td>
			<td style="border: 1px solid black;"><code>[1, 2, 5, 3]</code></td>
			<td style="border: 1px solid black;">11</td>
			<td style="border: 1px solid black;">5.5</td>
			<td style="border: 1px solid black;">Sum from <code>3 &rarr; 1 = 1 &lt; 5.5</code>.<br />
			Sum from <code>3 &rarr; 0 = 1 + 2 = 3 &lt; 5.5</code>.<br />
			Sum from <code>3 &rarr; 2 = 1 + 2 + 5 = 8 &gt;= 5.5</code>, median is node 2.</td>
			<td style="border: 1px solid black;">2</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[1, 2]</code></td>
			<td style="border: 1px solid black;"><code>1 &rarr; 0 &rarr; 2</code></td>
			<td style="border: 1px solid black;"><code>[2, 5]</code></td>
			<td style="border: 1px solid black;">7</td>
			<td style="border: 1px solid black;">3.5</td>
			<td style="border: 1px solid black;">
			<p>Sum from <code>1 &rarr; 0 = 2 &lt; 3.5</code>.<br />
			Sum from <code>1 &rarr; 2 = 2 + 5 = 7 &gt;= 3.5</code>, median is node 2.</p>
			</td>
			<td style="border: 1px solid black;">2</td>
		</tr>
	</tbody>
</table>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i] == [u<sub>i</sub>, v<sub>i</sub>, w<sub>i</sub>]</code></li>
	<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt; n</code></li>
	<li><code>1 &lt;= w<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>queries[j] == [u<sub>j</sub>, v<sub>j</sub>]</code></li>
	<li><code>0 &lt;= u<sub>j</sub>, v<sub>j</sub> &lt; n</code></li>
	<li>The input is generated such that <code>edges</code> represents a valid tree.</li>
</ul>


## Hints

1. Use binary lifting and lowest common ancestor.
2. Let the query nodes be <code>u</code> and <code>v</code>, with lowest common ancestor <code>l</code> and total path weight <code>tot</code>.
3. If the median lies on the path from <code>u</code> up to <code>l</code>: find the first node where <code>2 * sum >= tot</code> (equivalently, the last where <code>2 * sum < tot</code> and move one node above).
4. Otherwise, it lies on the path from <code>v</code> up to <code>l</code>: use the same <code>2 * sum >= tot</code> criterion as you climb.
5. In both cases, binary lifting with sparse tables lets you jump by powers of two while tracking cumulative weights to locate the weighted median in O(log n)

## Solution

```python3
from typing import List

class Solution:
    def findMedian(self, n: int, edges: List[List[int]], queries: List[List[int]]) -> List[int]:
        adj = [[] for _ in range(n)]
        for u, v, w in edges:
            adj[u].append((v, w))
            adj[v].append((u, w))

        LOG = 20
        up = [[-1] * LOG for _ in range(n)]
        dist = [0] * n
        depth = [0] * n

        stack = [0]
        visited = [False] * n
        visited[0] = True
        while stack:
            u = stack.pop()
            for v, w in adj[u]:
                if not visited[v]:
                    visited[v] = True
                    depth[v] = depth[u] + 1
                    dist[v] = dist[u] + w
                    up[v][0] = u
                    for i in range(1, LOG):
                        if up[v][i-1] != -1:
                            up[v][i] = up[up[v][i-1]][i-1]
                        else:
                            up[v][i] = -1
                    stack.append(v)

        def get_lca(u: int, v: int) -> int:
            if depth[u] < depth[v]:
                u, v = v, u
            diff = depth[u] - depth[v]
            for i in range(LOG):
                if (diff >> i) & 1:
                    u = up[u][i]
            if u == v:
                return u
            for i in range(LOG-1, -1, -1):
                if up[u][i] != up[v][i]:
                    u = up[u][i]
                    v = up[v][i]
            return up[u][0]

        res = []
        for u, v in queries:
            if u == v:
                res.append(u)
                continue
            lca = get_lca(u, v)
            W = dist[u] + dist[v] - 2 * dist[lca]
            target = (W + 1) // 2
            dist_u_lca = dist[u] - dist[lca]

            if target <= dist_u_lca:
                cur = u
                d = 0
                for i in range(LOG-1, -1, -1):
                    if up[cur][i] != -1:
                        w = dist[cur] - dist[up[cur][i]]
                        if d + w < target:
                            d += w
                            cur = up[cur][i]
                res.append(up[cur][0])
            else:
                needed_from_lca = target - dist_u_lca
                max_from_v = (dist[v] - dist[lca]) - needed_from_lca
                cur = v
                d = 0
                for i in range(LOG-1, -1, -1):
                    if up[cur][i] != -1:
                        w = dist[cur] - dist[up[cur][i]]
                        if d + w <= max_from_v:
                            d += w
                            cur = up[cur][i]
                res.append(cur)
        return res
```