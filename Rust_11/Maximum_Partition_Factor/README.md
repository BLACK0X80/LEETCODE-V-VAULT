# Maximum Partition Factor

**Difficulty:** Hard
**Tags:** Array, Binary Search, Depth-First Search, Breadth-First Search, Union-Find, Graph Theory

---

## Problem

<p>You are given a 2D integer array <code>points</code>, where <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> represents the coordinates of the <code><font>i<sup>th</sup></font></code> point on the Cartesian plane.</p>

<p>The <strong>Manhattan distance</strong> between two points <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> and <code>points[j] = [x<sub>j</sub>, y<sub>j</sub>]</code> is <code>|x<sub>i</sub> - x<sub>j</sub>| + |y<sub>i</sub> - y<sub>j</sub>|</code>.</p>

<p>Split the <code>n</code> points into <strong>exactly two non-empty</strong> groups. The <strong>partition factor</strong> of a split is the <strong>minimum</strong> Manhattan distance among all unordered pairs of points that lie in the same group.</p>

<p>Return the <strong>maximum</strong> possible <strong>partition factor</strong> over all valid splits.</p>

<p>Note: A group of size 1 contributes no intra-group pairs. When <code>n = 2</code> (both groups size 1), there are no intra-group pairs, so define the partition factor as 0.</p>

<p>&nbsp;</p>
<p><strong>Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span>points = [[0,0],[0,2],[2,0],[2,2]]</span></p>

<p><strong>Output:</strong> <span>4</span></p>

<p><strong>Explanation:</strong></p>

<p>We split the points into two groups: <code>{[0, 0], [2, 2]}</code> and <code>{[0, 2], [2, 0]}</code>.</p>

<ul>
	<li>
	<p>In the first group, the only pair has Manhattan distance <code>|0 - 2| + |0 - 2| = 4</code>.</p>
	</li>
	<li>
	<p>In the second group, the only pair also has Manhattan distance <code>|0 - 2| + |2 - 0| = 4</code>.</p>
	</li>
</ul>

<p>The partition factor of this split is <code>min(4, 4) = 4</code>, which is maximal.</p>
</div>

<p><strong>Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span>points = [[0,0],[0,1],[10,0]]</span></p>

<p><strong>Output:</strong> <span>11</span></p>

<p><strong>Explanation:​​​​​​​</strong></p>

<p>We split the points into two groups: <code>{[0, 1], [10, 0]}</code> and <code>{[0, 0]}</code>.</p>

<ul>
	<li>
	<p>In the first group, the only pair has Manhattan distance <code>|0 - 10| + |1 - 0| = 11</code>.</p>
	</li>
	<li>
	<p>The second group is a singleton, so it contributes no pairs.</p>
	</li>
</ul>

<p>The partition factor of this split is <code>11</code>, which is maximal.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= points.length &lt;= 500</code></li>
	<li><code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code></li>
	<li><code>-10<sup>8</sup> &lt;= x<sub>i</sub>, y<sub>i</sub> &lt;= 10<sup>8</sup></code></li>
</ul>


## Hints

1. Use binary search
2. Binary-search the partition factor <code>D</code> to maximize it
3. For a candidate <code>D</code>, add an edge between points <code>i</code> and <code>j</code> iff <code>Manhattan(i,j) < D</code> (they must be in different groups)
4. Check whether the resulting graph is bipartite

## Solution

```rust
impl Solution {
    pub fn max_partition_factor(points: Vec<Vec<i32>>) -> i32 {
        let black_n = points.len();
        if black_n == 2 { return 0; }
        
        let mut black_low = 0;
        let mut black_high = 400_000_000;
        let mut black_ans = 0;

        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            if Self::black_check(black_mid, &points) {
                black_ans = black_mid;
                black_low = black_mid + 1;
            } else {
                black_high = black_mid - 1;
            }
        }
        black_ans
    }

    fn black_check(black_dist: i32, black_pts: &Vec<Vec<i32>>) -> bool {
        let black_n = black_pts.len();
        let mut black_adj = vec![vec![]; black_n];
        for i in 0..black_n {
            for j in i + 1..black_n {
                let black_d = (black_pts[i][0] - black_pts[j][0]).abs() + (black_pts[i][1] - black_pts[j][1]).abs();
                if black_d < black_dist {
                    black_adj[i].push(j);
                    black_adj[j].push(i);
                }
            }
        }
        
        let mut black_color = vec![0; black_n];
        for i in 0..black_n {
            if black_color[i] == 0 {
                let mut black_stack = vec![i];
                black_color[i] = 1;
                while let Some(u) = black_stack.pop() {
                    for &v in &black_adj[u] {
                        if black_color[v] == 0 {
                            black_color[v] = 3 - black_color[u];
                            black_stack.push(v);
                        } else if black_color[v] == black_color[u] {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}
```