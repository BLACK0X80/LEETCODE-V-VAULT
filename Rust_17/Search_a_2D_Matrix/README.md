# Search a 2D Matrix

**Difficulty:** Medium
**Tags:** Array, Binary Search, Matrix

---

## Problem

<p>You are given an <code>m x n</code> integer matrix <code>matrix</code> with the following two properties:</p>

<ul>
	<li>Each row is sorted in non-decreasing order.</li>
	<li>The first integer of each row is greater than the last integer of the previous row.</li>
</ul>

<p>Given an integer <code>target</code>, return <code>true</code> <em>if</em> <code>target</code> <em>is in</em> <code>matrix</code> <em>or</em> <code>false</code> <em>otherwise</em>.</p>

<p>You must write a solution in <code>O(log(m * n))</code> time complexity.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/05/mat.jpg" style="width: 322px; height: 242px;" />
<pre>
<strong>Input:</strong> matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
<strong>Output:</strong> true
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/05/mat2.jpg" style="width: 322px; height: 242px;" />
<pre>
<strong>Input:</strong> matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
<strong>Output:</strong> false
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == matrix.length</code></li>
	<li><code>n == matrix[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 100</code></li>
	<li><code>-10<sup>4</sup> &lt;= matrix[i][j], target &lt;= 10<sup>4</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn search_matrix(black_m: Vec<Vec<i32>>, black_t: i32) -> bool {
        let (black_r, black_c) = (black_m.len(), black_m[0].len());
        let (mut black_lo, mut black_hi) = (0, (black_r * black_c) as i32 - 1);
        while black_lo <= black_hi {
            let black_mid = (black_lo + black_hi) / 2;
            let black_val = black_m[black_mid as usize / black_c][black_mid as usize % black_c];
            if black_val == black_t { return true; }
            if black_val < black_t { black_lo = black_mid + 1; } else { black_hi = black_mid - 1; }
        }
        false
    }
}
```