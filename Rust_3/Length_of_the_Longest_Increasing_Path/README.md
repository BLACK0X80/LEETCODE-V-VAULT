# Length of the Longest Increasing Path

**Difficulty:** Hard
**Tags:** Array, Binary Search, Sorting

---

## Problem

<p>You are given a 2D array of integers <code>coordinates</code> of length <code>n</code> and an integer <code>k</code>, where <code>0 &lt;= k &lt; n</code>.</p>

<p><code>coordinates[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> indicates the point <code>(x<sub>i</sub>, y<sub>i</sub>)</code> in a 2D plane.</p>

<p>An <strong>increasing path</strong> of length <code>m</code> is defined as a list of points <code>(x<sub>1</sub>, y<sub>1</sub>)</code>, <code>(x<sub>2</sub>, y<sub>2</sub>)</code>, <code>(x<sub>3</sub>, y<sub>3</sub>)</code>, ..., <code>(x<sub>m</sub>, y<sub>m</sub>)</code> such that:</p>

<ul>
	<li><code>x<sub>i</sub> &lt; x<sub>i + 1</sub></code> and <code>y<sub>i</sub> &lt; y<sub>i + 1</sub></code> for all <code>i</code> where <code>1 &lt;= i &lt; m</code>.</li>
	<li><code>(x<sub>i</sub>, y<sub>i</sub>)</code> is in the given coordinates for all <code>i</code> where <code>1 &lt;= i &lt;= m</code>.</li>
</ul>

<p>Return the <strong>maximum</strong> length of an <strong>increasing path</strong> that contains <code>coordinates[k]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">coordinates = [[3,1],[2,2],[4,1],[0,0],[5,3]], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p><code>(0, 0)</code>, <code>(2, 2)</code>, <code>(5, 3)</code><!-- notionvc: 082cee9e-4ce5-4ede-a09d-57001a72141d --> is the longest increasing path that contains <code>(2, 2)</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">coordinates = [[2,1],[7,0],[5,6]], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p><code>(2, 1)</code>, <code>(5, 6)</code> is the longest increasing path that contains <code>(5, 6)</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == coordinates.length &lt;= 10<sup>5</sup></code></li>
	<li><code>coordinates[i].length == 2</code></li>
	<li><code>0 &lt;= coordinates[i][0], coordinates[i][1] &lt;= 10<sup>9</sup></code></li>
	<li>All elements in <code>coordinates</code> are <strong>distinct</strong>.<!-- notionvc: 6e412fc2-f9dd-4ba2-b796-5e802a2b305a --><!-- notionvc: c2cf5618-fe99-4909-9b4c-e6b068be22a6 --></li>
	<li><code>0 &lt;= k &lt;= n - 1</code></li>
</ul>


## Hints

1. Only keep coordinates with both <code>x</code> and <code>y</code> being strictly less than <code>coordinates[k]</code>.
2. Sort them by <code>x</code>’s, in the case of equal, the <code>y</code> values should be decreasing.
3. Calculate LIS only using <code>y</code> values.
4. Do the same for coordinates with both <code>x</code> and <code>y</code> being strictly larger than <code>coordinates[k]</code>.

## Solution

```rust
impl Solution {
    pub fn max_path_length(mut black_coords: Vec<Vec<i32>>, black_k: i32) -> i32 {
        let black_target = black_coords[black_k as usize].clone();
        
        let mut black_calc = |black_pts: Vec<&Vec<i32>>| -> i32 {
            let mut black_pts = black_pts;
            black_pts.sort_by(|black_a, black_b| black_a[0].cmp(&black_b[0]).then(black_b[1].cmp(&black_a[1])));
            let mut black_tails = Vec::new();
            for black_p in black_pts {
                let black_y = black_p[1];
                let black_pos = black_tails.binary_search(&black_y).unwrap_or_else(|black_e| black_e);
                if black_pos < black_tails.len() { black_tails[black_pos] = black_y; }
                else { black_tails.push(black_y); }
            }
            black_tails.len() as i32
        };

        let black_before: Vec<&Vec<i32>> = black_coords.iter().filter(|black_p| black_p[0] < black_target[0] && black_p[1] < black_target[1]).collect();
        let black_after: Vec<&Vec<i32>> = black_coords.iter().filter(|black_p| black_p[0] > black_target[0] && black_p[1] > black_target[1]).collect();

        black_calc(black_before) + 1 + black_calc(black_after)
    }
}
```