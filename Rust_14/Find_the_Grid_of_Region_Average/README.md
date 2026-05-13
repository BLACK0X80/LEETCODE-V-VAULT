# Find the Grid of Region Average

**Difficulty:** Medium
**Tags:** Array, Matrix

---

## Problem

<p>You are given <code>m x n</code> grid <code>image</code> which represents a grayscale image, where <code>image[i][j]</code> represents a pixel with intensity in the range <code>[0..255]</code>. You are also given a <strong>non-negative</strong> integer <code>threshold</code>.</p>

<p>Two pixels are <strong>adjacent</strong> if they share an edge.</p>

<p>A <strong>region</strong> is a <code>3 x 3</code> subgrid where the <strong>absolute difference</strong> in intensity between any two <strong>adjacent</strong> pixels is <strong>less than or equal to</strong> <code>threshold</code>.</p>

<p>All pixels in a region belong to that region, note that a pixel can belong to <strong>multiple</strong> regions.</p>

<p>You need to calculate a <code>m x n</code> grid <code>result</code>, where <code>result[i][j]</code> is the <strong>average</strong> intensity of the regions to which <code>image[i][j]</code> belongs, <strong>rounded down</strong> to the nearest integer. If <code>image[i][j]</code> belongs to multiple regions, <code>result[i][j]</code> is the <strong>average </strong>of the<strong> rounded-down average </strong>intensities of these regions, <strong>rounded down</strong> to the nearest integer. If <code>image[i][j]</code> does<strong> not</strong> belong to any region, <code>result[i][j]</code> is <strong>equal to</strong> <code>image[i][j]</code>.</p>

<p>Return the grid <code>result</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">image = [[5,6,7,10],[8,9,10,10],[11,12,13,10]], threshold = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">[[9,9,9,9],[9,9,9,9],[9,9,9,9]]</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2023/12/21/example0corrected.png" style="width: 832px; height: 275px;" /></p>

<p>There are two regions as illustrated above. The average intensity of the first region is 9, while the average intensity of the second region is 9.67 which is rounded down to 9. The average intensity of both of the regions is (9 + 9) / 2 = 9. As all the pixels belong to either region 1, region 2, or both of them, the intensity of every pixel in the result is 9.</p>

<p>Please note that the rounded-down values are used when calculating the average of multiple regions, hence the calculation is done using 9 as the average intensity of region 2, not 9.67.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">image = [[10,20,30],[15,25,35],[20,30,40],[25,35,45]], threshold = 12</span></p>

<p><strong>Output:</strong> <span class="example-io">[[25,25,25],[27,27,27],[27,27,27],[30,30,30]]</span></p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2023/12/21/example1corrected.png" /></p>

<p>There are two regions as illustrated above. The average intensity of the first region is 25, while the average intensity of the second region is 30. The average intensity of both of the regions is (25 + 30) / 2 = 27.5 which is rounded down to 27.</p>

<p>All the pixels in row 0 of the image belong to region 1, hence all the pixels in row 0 in the result are 25. Similarly, all the pixels in row 3 in the result are 30. The pixels in rows 1 and 2 of the image belong to region 1 and region 2, hence their assigned value is 27 in the result.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">image = [[5,6,7],[8,9,10],[11,12,13]], threshold = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">[[5,6,7],[8,9,10],[11,12,13]]</span></p>

<p><strong>Explanation:</strong></p>

<p>There is only one <code>3 x 3</code> subgrid, while it does not have the condition on difference of adjacent pixels, for example, the difference between <code>image[0][0]</code> and <code>image[1][0]</code> is <code>|5 - 8| = 3 &gt; threshold = 1</code>. None of them belong to any valid regions, so the <code>result</code> should be the same as <code>image</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= n, m &lt;= 500</code></li>
	<li><code>0 &lt;= image[i][j] &lt;= 255</code></li>
	<li><code>0 &lt;= threshold &lt;= 255</code></li>
</ul>


## Hints

1. Try all the <code>3 * 3</code> sub-grids to find all the regions.
2. Keep two 2-D arrays <code>sum</code> and <code>num</code>, for each position <code>(x, y)</code> in a region, increase <code>sum[x][y]</code> by the average sum of the region and increase <code>num[x][y]</code> by <code>1</code>.
3. For each position (x, y), <code>sum[x][y] / num[x][y]</code> is the answer. Note when <code>num[x][y] == 0</code>, we use the original value in <code>image</code> instead.

## Solution

```rust
impl Solution { pub fn result_grid(black_img: Vec<Vec<i32>>, black_t: i32) -> Vec<Vec<i32>> { let (black_m, black_n) = (black_img.len(), black_img[0].len()); let (mut black_sum, mut black_count) = (vec![vec![0; black_n]; black_m], vec![vec![0; black_n]; black_m]); for i in 0..=black_m - 3 { for j in 0..=black_n - 3 { let mut black_ok = true; for r in i..i+3 { for c in j..j+3 { if (r + 1 < i + 3 && (black_img[r][c] - black_img[r+1][c]).abs() > black_t) || (c + 1 < j + 3 && (black_img[r][c] - black_img[r][c+1]).abs() > black_t) { black_ok = false; break; } } if !black_ok { break; } } if black_ok { let black_avg = (i..i+3).map(|r| (j..j+3).map(|c| black_img[r][c]).sum::<i32>()).sum::<i32>() / 9; for r in i..i+3 { for c in j..j+3 { black_sum[r][c] += black_avg; black_count[r][c] += 1; } } } } } let mut black_res = vec![vec![0; black_n]; black_m]; for i in 0..black_m { for j in 0..black_n { black_res[i][j] = if black_count[i][j] > 0 { black_sum[i][j] / black_count[i][j] } else { black_img[i][j] }; } } black_res } }
```