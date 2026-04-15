# Build a Matrix With Conditions

**Difficulty:** Hard
**Tags:** Array, Graph Theory, Topological Sort, Matrix

---

## Problem

<p>You are given a <strong>positive</strong> integer <code>k</code>. You are also given:</p>

<ul>
	<li>a 2D integer array <code>rowConditions</code> of size <code>n</code> where <code>rowConditions[i] = [above<sub>i</sub>, below<sub>i</sub>]</code>, and</li>
	<li>a 2D integer array <code>colConditions</code> of size <code>m</code> where <code>colConditions[i] = [left<sub>i</sub>, right<sub>i</sub>]</code>.</li>
</ul>

<p>The two arrays contain integers from <code>1</code> to <code>k</code>.</p>

<p>You have to build a <code>k x k</code> matrix that contains each of the numbers from <code>1</code> to <code>k</code> <strong>exactly once</strong>. The remaining cells should have the value <code>0</code>.</p>

<p>The matrix should also satisfy the following conditions:</p>

<ul>
	<li>The number <code>above<sub>i</sub></code> should appear in a <strong>row</strong> that is strictly <strong>above</strong> the row at which the number <code>below<sub>i</sub></code> appears for all <code>i</code> from <code>0</code> to <code>n - 1</code>.</li>
	<li>The number <code>left<sub>i</sub></code> should appear in a <strong>column</strong> that is strictly <strong>left</strong> of the column at which the number <code>right<sub>i</sub></code> appears for all <code>i</code> from <code>0</code> to <code>m - 1</code>.</li>
</ul>

<p>Return <em><strong>any</strong> matrix that satisfies the conditions</em>. If no answer exists, return an empty matrix.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/07/06/gridosdrawio.png" style="width: 211px; height: 211px;" />
<pre>
<strong>Input:</strong> k = 3, rowConditions = [[1,2],[3,2]], colConditions = [[2,1],[3,2]]
<strong>Output:</strong> [[3,0,0],[0,0,1],[0,2,0]]
<strong>Explanation:</strong> The diagram above shows a valid example of a matrix that satisfies all the conditions.
The row conditions are the following:
- Number 1 is in row <u>1</u>, and number 2 is in row <u>2</u>, so 1 is above 2 in the matrix.
- Number 3 is in row <u>0</u>, and number 2 is in row <u>2</u>, so 3 is above 2 in the matrix.
The column conditions are the following:
- Number 2 is in column <u>1</u>, and number 1 is in column <u>2</u>, so 2 is left of 1 in the matrix.
- Number 3 is in column <u>0</u>, and number 2 is in column <u>1</u>, so 3 is left of 2 in the matrix.
Note that there may be multiple correct answers.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> k = 3, rowConditions = [[1,2],[2,3],[3,1],[2,3]], colConditions = [[2,1]]
<strong>Output:</strong> []
<strong>Explanation:</strong> From the first two conditions, 3 has to be below 1 but the third conditions needs 3 to be above 1 to be satisfied.
No matrix can satisfy all the conditions, so we return the empty matrix.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= k &lt;= 400</code></li>
	<li><code>1 &lt;= rowConditions.length, colConditions.length &lt;= 10<sup>4</sup></code></li>
	<li><code>rowConditions[i].length == colConditions[i].length == 2</code></li>
	<li><code>1 &lt;= above<sub>i</sub>, below<sub>i</sub>, left<sub>i</sub>, right<sub>i</sub> &lt;= k</code></li>
	<li><code>above<sub>i</sub> != below<sub>i</sub></code></li>
	<li><code>left<sub>i</sub> != right<sub>i</sub></code></li>
</ul>


## Hints

1. Can you think of the problem in terms of graphs?
2. What algorithm allows you to find the order of nodes in a graph?

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn build_matrix(black1: i32, black2: Vec<Vec<i32>>, black3: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let black4 = black1 as usize;

        let black5 = |black6: &Vec<Vec<i32>>| -> Option<Vec<usize>> {
            let mut black7 = vec![vec![]; black4 + 1];
            let mut black8 = vec![0; black4 + 1];
            for black9 in black6 {
                black7[black9[0] as usize].push(black9[1] as usize);
                black8[black9[1] as usize] += 1;
            }

            let mut black10 = VecDeque::new();
            for black11 in 1..=black4 {
                if black8[black11] == 0 {
                    black10.push_back(black11);
                }
            }

            let mut black12 = Vec::new();
            while let Some(black13) = black10.pop_front() {
                black12.push(black13);
                for &black14 in &black7[black13] {
                    black8[black14] -= 1;
                    if black8[black14] == 0 {
                        black10.push_back(black14);
                    }
                }
            }

            if black12.len() == black4 { Some(black12) } else { None }
        };

        let black15 = match black5(&black2) {
            Some(black16) => black16,
            None => return vec![],
        };

        let black17 = match black5(&black3) {
            Some(black18) => black18,
            None => return vec![],
        };

        let mut black19 = vec![0; black4 + 1];
        for (black20, &black21) in black17.iter().enumerate() {
            black19[black21] = black20;
        }

        let mut black22 = vec![vec![0; black4]; black4];
        for (black23, &black24) in black15.iter().enumerate() {
            black22[black23][black19[black24]] = black24 as i32;
        }

        black22
    }
}
```