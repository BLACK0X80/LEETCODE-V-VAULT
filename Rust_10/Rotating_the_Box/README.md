# Rotating the Box

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Matrix

---

## Problem

<p>You are given an <code>m x n</code> matrix of characters <code>boxGrid</code> representing a side-view of a box. Each cell of the box is one of the following:</p>

<ul>
	<li>A stone <code>&#39;#&#39;</code></li>
	<li>A stationary obstacle <code>&#39;*&#39;</code></li>
	<li>Empty <code>&#39;.&#39;</code></li>
</ul>

<p>The box is rotated <strong>90 degrees clockwise</strong>, causing some of the stones to fall due to gravity. Each stone falls down until it lands on an obstacle, another stone, or the bottom of the box. Gravity <strong>does not</strong> affect the obstacles&#39; positions, and the inertia from the box&#39;s rotation <strong>does not </strong>affect the stones&#39; horizontal positions.</p>

<p>It is <strong>guaranteed</strong> that each stone in <code>boxGrid</code> rests on an obstacle, another stone, or the bottom of the box.</p>

<p>Return <em>an </em><code>n x m</code><em> matrix representing the box after the rotation described above</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcodewithstones.png" style="width: 300px; height: 150px;" /></p>

<pre>
<strong>Input:</strong> boxGrid = [[&quot;#&quot;,&quot;.&quot;,&quot;#&quot;]]
<strong>Output:</strong> [[&quot;.&quot;],
&nbsp;        [&quot;#&quot;],
&nbsp;        [&quot;#&quot;]]
</pre>

<p><strong class="example">Example 2:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcode2withstones.png" style="width: 375px; height: 195px;" /></p>

<pre>
<strong>Input:</strong> boxGrid = [[&quot;#&quot;,&quot;.&quot;,&quot;*&quot;,&quot;.&quot;],
&nbsp;             [&quot;#&quot;,&quot;#&quot;,&quot;*&quot;,&quot;.&quot;]]
<strong>Output:</strong> [[&quot;#&quot;,&quot;.&quot;],
&nbsp;        [&quot;#&quot;,&quot;#&quot;],
&nbsp;        [&quot;*&quot;,&quot;*&quot;],
&nbsp;        [&quot;.&quot;,&quot;.&quot;]]
</pre>

<p><strong class="example">Example 3:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcode3withstone.png" style="width: 400px; height: 218px;" /></p>

<pre>
<strong>Input:</strong> boxGrid = [[&quot;#&quot;,&quot;#&quot;,&quot;*&quot;,&quot;.&quot;,&quot;*&quot;,&quot;.&quot;],
&nbsp;             [&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;*&quot;,&quot;.&quot;,&quot;.&quot;],
&nbsp;             [&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;.&quot;,&quot;#&quot;,&quot;.&quot;]]
<strong>Output:</strong> [[&quot;.&quot;,&quot;#&quot;,&quot;#&quot;],
&nbsp;        [&quot;.&quot;,&quot;#&quot;,&quot;#&quot;],
&nbsp;        [&quot;#&quot;,&quot;#&quot;,&quot;*&quot;],
&nbsp;        [&quot;#&quot;,&quot;*&quot;,&quot;.&quot;],
&nbsp;        [&quot;#&quot;,&quot;.&quot;,&quot;*&quot;],
&nbsp;        [&quot;#&quot;,&quot;.&quot;,&quot;.&quot;]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == boxGrid.length</code></li>
	<li><code>n == boxGrid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 500</code></li>
	<li><code>boxGrid[i][j]</code> is either <code>&#39;#&#39;</code>, <code>&#39;*&#39;</code>, or <code>&#39;.&#39;</code>.</li>
</ul>


## Hints

1. Rotate the box using the relation rotatedBox[i][j] = box[m - 1 - j][i].
2. Start iterating from the bottom of the box and for each empty cell check if there is any stone above it with no obstacles between them.

## Solution

```rust
impl Solution { pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> { let (black_m, black_n) = (box_grid.len(), box_grid[0].len()); let mut black_b = box_grid.clone(); black_b.iter_mut().for_each(|black_r| { let mut black_empty = black_n - 1; (0..black_n).rev().for_each(|black_j| { if black_r[black_j] == '#' { black_r[black_j] = '.'; black_r[black_empty] = '#'; black_empty -= 1; } else if black_r[black_j] == '*' { black_empty = if black_j > 0 { black_j - 1 } else { 0 }; } else if black_r[black_j] == '.' && black_r[black_empty] != '.' { black_empty = black_j; } }); }); (0..black_n).map(|black_j| (0..black_m).rev().map(|black_i| black_b[black_i][black_j]).collect()).collect() } }
```