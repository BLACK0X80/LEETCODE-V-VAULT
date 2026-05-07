# Find the Number of Copy Arrays

**Difficulty:** Medium
**Tags:** Array, Math

---

## Problem

<p>You are given an array <code>original</code> of length <code>n</code> and a 2D array <code>bounds</code> of length <code>n x 2</code>, where <code>bounds[i] = [u<sub>i</sub>, v<sub>i</sub>]</code>.</p>

<p>You need to find the number of <strong>possible</strong> arrays <code>copy</code> of length <code>n</code> such that:</p>

<ol>
	<li><code>(copy[i] - copy[i - 1]) == (original[i] - original[i - 1])</code> for <code>1 &lt;= i &lt;= n - 1</code>.</li>
	<li><code>u<sub>i</sub> &lt;= copy[i] &lt;= v<sub>i</sub></code> for <code>0 &lt;= i &lt;= n - 1</code>.</li>
</ol>

<p>Return the number of such arrays.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">original = [1,2,3,4], bounds = [[1,2],[2,3],[3,4],[4,5]]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The possible arrays are:</p>

<ul>
	<li><code>[1, 2, 3, 4]</code></li>
	<li><code>[2, 3, 4, 5]</code></li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">original = [1,2,3,4], bounds = [[1,10],[2,9],[3,8],[4,7]]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The possible arrays are:</p>

<ul>
	<li><code>[1, 2, 3, 4]</code></li>
	<li><code>[2, 3, 4, 5]</code></li>
	<li><code>[3, 4, 5, 6]</code></li>
	<li><code>[4, 5, 6, 7]</code></li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">original = [1,2,1,2], bounds = [[1,1],[2,3],[3,3],[2,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>No array is possible.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n == original.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= original[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>bounds.length == n</code></li>
	<li><code>bounds[i].length == 2</code></li>
	<li><code>1 &lt;= bounds[i][0] &lt;= bounds[i][1] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. <code>copy[0]</code> uniquely determines all other values.
2. Possible values for <code>copy[0]</code> are in <code>[u[0], v[0]]</code>.
3. From left to right, compute valid ranges for each index by intersecting bounds with the previous range.
4. The answer is the size of the valid range for the last index.

## Solution

```rust
impl Solution { pub fn count_arrays(original: Vec<i32>, bounds: Vec<Vec<i32>>) -> i32 { let (mut black_low, mut black_high) = (bounds[0][0], bounds[0][1]); for i in 1..original.len() { let black_diff = original[i] - original[i-1]; black_low = (black_low + black_diff).max(bounds[i][0]); black_high = (black_high + black_diff).min(bounds[i][1]); } 0.max(black_high - black_low + 1) } }
```