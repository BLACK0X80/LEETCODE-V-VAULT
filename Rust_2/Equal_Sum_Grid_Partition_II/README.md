# Equal Sum Grid Partition II

**Difficulty:** Hard
**Tags:** Array, Hash Table, Matrix, Enumeration, Prefix Sum

---

## Problem

<p>You are given an <code>m x n</code> matrix <code>grid</code> of positive integers. Your task is to determine if it is possible to make <strong>either one horizontal or one vertical cut</strong> on the grid such that:</p>

<ul>
	<li>Each of the two resulting sections formed by the cut is <strong>non-empty</strong>.</li>
	<li>The sum of elements in both sections is <b>equal</b>, or can be made equal by discounting <strong>at most</strong> one single cell in total (from either section).</li>
	<li>If a cell is discounted, the rest of the section must <strong>remain connected</strong>.</li>
</ul>

<p>Return <code>true</code> if such a partition exists; otherwise, return <code>false</code>.</p>

<p><strong>Note:</strong> A section is <strong>connected</strong> if every cell in it can be reached from any other cell by moving up, down, left, or right through other cells in the section.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,4],[2,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">true</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/03/30/lc.jpeg" style="height: 180px; width: 180px;" /></p>

<ul>
	<li>A horizontal cut after the first row gives sums <code>1 + 4 = 5</code> and <code>2 + 3 = 5</code>, which are equal. Thus, the answer is <code>true</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,2],[3,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">true</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/04/01/chatgpt-image-apr-1-2025-at-05_28_12-pm.png" style="height: 180px; width: 180px;" /></p>

<ul>
	<li>A vertical cut after the first column gives sums <code>1 + 3 = 4</code> and <code>2 + 4 = 6</code>.</li>
	<li>By discounting 2 from the right section (<code>6 - 2 = 4</code>), both sections have equal sums and remain connected. Thus, the answer is <code>true</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,2,4],[2,3,5]]</span></p>

<p><strong>Output:</strong> <span class="example-io">false</span></p>

<p><strong>Explanation:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2025/04/01/chatgpt-image-apr-2-2025-at-02_50_29-am.png" style="height: 180px; width: 180px;" /></strong></p>

<ul>
	<li>A horizontal cut after the first row gives <code>1 + 2 + 4 = 7</code> and <code>2 + 3 + 5 = 10</code>.</li>
	<li>By discounting 3 from the bottom section (<code>10 - 3 = 7</code>), both sections have equal sums, but they do not remain connected as it splits the bottom section into two parts (<code>[2]</code> and <code>[5]</code>). Thus, the answer is <code>false</code>.</li>
</ul>
</div>

<p><strong class="example">Example 4:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[4,1,8],[3,2,6]]</span></p>

<p><strong>Output:</strong> <span class="example-io">false</span></p>

<p><strong>Explanation:</strong></p>

<p>No valid cut exists, so the answer is <code>false</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m == grid.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= n == grid[i].length &lt;= 10<sup>5</sup></code></li>
	<li><code>2 &lt;= m * n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= grid[i][j] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. In a grid (or any subgrid), when can a section be disconnected? Can disconnected components occur if the section spans more than one row and more than one column?
2. Handle single rows or single columns separately. For all other partitions, maintain the sums and value frequencies of each section to check whether removing at most one element from one section can make the two sums equal.

## Solution

```rust
impl Solution {
    pub fn can_partition_grid(mut grid: Vec<Vec<i32>>) -> bool {
        let total: i64 = grid.iter().flat_map(|r| r.iter()).map(|&x| x as i64).sum();

        for _ in 0..4 {
            let m = grid.len();
            let n = grid[0].len();

            if m >= 2 {
                let mut sum = 0i64;
                let mut exist = std::collections::HashSet::new();
                exist.insert(0i64);

                if n == 1 {
                    for i in 0..m - 1 {
                        sum += grid[i][0] as i64;
                        let tag = sum * 2 - total;
                        if tag == 0 || tag == grid[0][0] as i64 || tag == grid[i][0] as i64 {
                            return true;
                        }
                    }
                } else {
                    for i in 0..m - 1 {
                        for j in 0..n {
                            exist.insert(grid[i][j] as i64);
                            sum += grid[i][j] as i64;
                        }
                        let tag = sum * 2 - total;
                        if i == 0 {
                            if tag == 0 || tag == grid[0][0] as i64 || tag == grid[0][n-1] as i64 {
                                return true;
                            }
                        } else if exist.contains(&tag) {
                            return true;
                        }
                    }
                }
            }

            grid = rotate(grid);
        }

        false
    }
}

fn rotate(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut tmp = vec![vec![0; m]; n];
    for i in 0..m {
        for j in 0..n {
            tmp[j][m - 1 - i] = grid[i][j];
        }
    }
    tmp
}
```