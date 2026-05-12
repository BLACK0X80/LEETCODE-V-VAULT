# Minimum Bitwise OR From Grid

**Difficulty:** Medium
**Tags:** Array, Greedy, Bit Manipulation, Matrix

---

## Problem

<p>You are given a 2D integer array <code>grid</code> of size <code>m x n</code>.</p>

<p>You must select <strong>exactly one</strong> integer from each row of the grid.</p>

<p>Return an integer denoting the <strong>minimum possible bitwise OR</strong> of the selected integers from each row.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,5],[2,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Choose 1 from the first row and 2 from the second row.</li>
	<li>The bitwise OR of <code>1 | 2 = 3</code>​​​​​​​, which is the minimum possible.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[3,5],[6,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Choose 5 from the first row and 4 from the second row.</li>
	<li>The bitwise OR of <code>5 | 4 = 5</code>​​​​​​​, which is the minimum possible.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[7,9,8]]</span></p>

<p><strong>Output:</strong> <span class="example-io">7</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Choosing 7 gives the minimum bitwise OR.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m == grid.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= n == grid[i].length &lt;= 10<sup>5</sup></code></li>
	<li><code>m * n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= grid[i][j] &lt;= 10<sup>5</sup>​​​​​​​</code></li>
</ul>


## Hints

1. Solve greedily, bit by bit from the most significant to the least significant
2. For a bit, check whether it is possible to exclude it from the OR by choosing, in every row, at least one number with that bit unset
3. Accumulate all bits that cannot be excluded; the final value is the minimum possible bitwise OR

## Solution

```rust
impl Solution { pub fn minimum_or(black_g: Vec<Vec<i32>>) -> i32 { let mut black_res = 0; for black_b in (0..20).rev() { let black_target = black_res; let mut black_ok = true; for black_row in &black_g { if !black_row.iter().any(|&black_v| (black_v | black_target | ( (1 << black_b) - 1 )) == (black_target | ( (1 << black_b) - 1 ))) { black_ok = false; break; } } if !black_ok { black_res |= 1 << black_b; } } black_res } }
```