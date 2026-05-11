# Length of Longest V-Shaped Diagonal Segment

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Memoization, Matrix

---

## Problem

<p>You are given a 2D integer matrix <code>grid</code> of size <code>n x m</code>, where each element is either <code>0</code>, <code>1</code>, or <code>2</code>.</p>

<p>A <strong>V-shaped diagonal segment</strong> is defined as:</p>

<ul>
	<li>The segment starts with <code>1</code>.</li>
	<li>The subsequent elements follow this infinite sequence: <code>2, 0, 2, 0, ...</code>.</li>
	<li>The segment:
	<ul>
		<li>Starts <strong>along</strong> a diagonal direction (top-left to bottom-right, bottom-right to top-left, top-right to bottom-left, or bottom-left to top-right).</li>
		<li>Continues the<strong> sequence</strong> in the same diagonal direction.</li>
		<li>Makes<strong> at most one clockwise 90-degree</strong><strong> turn</strong> to another diagonal direction while <strong>maintaining</strong> the sequence.</li>
	</ul>
	</li>
</ul>

<p>Return the <strong>length</strong> of the <strong>longest</strong> <strong>V-shaped diagonal segment</strong>. If no valid segment <em>exists</em>, return 0.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[2,2,1,2,2],[2,0,2,2,0],[2,0,1,1,0],[1,0,2,2,2],[2,0,0,2,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/12/09/matrix_1-2.jpg" style="width: 201px; height: 192px;" /></p>

<p>The longest V-shaped diagonal segment has a length of 5 and follows these coordinates: <code>(0,2) &rarr; (1,3) &rarr; (2,4)</code>, takes a <strong>90-degree clockwise turn</strong> at <code>(2,4)</code>, and continues as <code>(3,3) &rarr; (4,2)</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[2,2,2,2,2],[2,0,2,2,0],[2,0,1,1,0],[1,0,2,2,2],[2,0,0,2,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2024/12/09/matrix_2.jpg" style="width: 201px; height: 201px;" /></strong></p>

<p>The longest V-shaped diagonal segment has a length of 4 and follows these coordinates: <code>(2,3) &rarr; (3,2)</code>, takes a <strong>90-degree clockwise turn</strong> at <code>(3,2)</code>, and continues as <code>(2,1) &rarr; (1,0)</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,2,2,2,2],[2,2,2,2,0],[2,0,0,0,0],[0,0,2,2,2],[2,0,0,2,0]]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2024/12/09/matrix_3.jpg" style="width: 201px; height: 201px;" /></strong></p>

<p>The longest V-shaped diagonal segment has a length of 5 and follows these coordinates: <code>(0,0) &rarr; (1,1) &rarr; (2,2) &rarr; (3,3) &rarr; (4,4)</code>.</p>
</div>

<p><strong class="example">Example 4:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The longest V-shaped diagonal segment has a length of 1 and follows these coordinates: <code>(0,0)</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == grid.length</code></li>
	<li><code>m == grid[i].length</code></li>
	<li><code>1 &lt;= n, m &lt;= 500</code></li>
	<li><code>grid[i][j]</code> is either <code>0</code>, <code>1</code> or <code>2</code>.</li>
</ul>


## Hints

1. Use dynamic programming to determine the best point to make a 90-degree rotation in the diagonal path while maintaining the required sequence.
2. Represent dynamic programming states as <code>(row, col, currentDirection, hasMadeTurnYet)</code>. Track the current position, direction of traversal, and whether a turn has already been made, and take transitions accordingly to find the longest V-shaped diagonal segment.

## Solution

```rust
use std::collections::HashMap;
impl Solution {
    pub fn len_of_v_diagonal(black1: Vec<Vec<i32>>) -> i32 {
        let (black2, black3) = (black1.len() as i32, black1[0].len() as i32);
        let mut black4 = 0;
        let black5 = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
        fn black6(black7: i32, black8: i32, black9: usize, black10: i32, black11: i32, black12: &Vec<Vec<i32>>, black13: i32, black14: i32, black15: &[(i32, i32)], black16: &mut HashMap<(i32, i32, usize, i32, i32), i32>) -> i32 {
            if black7 < 0 || black8 < 0 || black7 >= black13 || black8 >= black14 || black12[black7 as usize][black8 as usize] != black11 { return 0; }
            if let Some(&black17) = black16.get(&(black7, black8, black9, black10, black11)) { return black17; }
            let black18 = if black11 == 2 { 0 } else { 2 };
            let mut black19 = 1 + black6(black7 + black15[black9].0, black8 + black15[black9].1, black9, black10, black18, black12, black13, black14, black15, black16);
            if black10 == 0 {
                let black20 = (black9 + 1) % 4;
                black19 = black19.max(1 + black6(black7 + black15[black20].0, black8 + black15[black20].1, black20, 1, black18, black12, black13, black14, black15, black16));
            }
            black16.insert((black7, black8, black9, black10, black11), black19);
            black19
        }
        let mut black21 = HashMap::new();
        for black22 in 0..black2 {
            for black23 in 0..black3 {
                if black1[black22 as usize][black23 as usize] == 1 {
                    for black24 in 0..4 { black4 = black4.max(1 + black6(black22 + black5[black24].0, black23 + black5[black24].1, black24, 0, 2, &black1, black2, black3, &black5, &mut black21)); }
                }
            }
        }
        black4
    }
}
```