# Kth Smallest Number in Multiplication Table

**Difficulty:** Hard
**Tags:** Math, Binary Search

---

## Problem

<p>Nearly everyone has used the <a href="https://en.wikipedia.org/wiki/Multiplication_table" target="_blank">Multiplication Table</a>. The multiplication table of size <code>m x n</code> is an integer matrix <code>mat</code> where <code>mat[i][j] == i * j</code> (<strong>1-indexed</strong>).</p>

<p>Given three integers <code>m</code>, <code>n</code>, and <code>k</code>, return <em>the </em><code>k<sup>th</sup></code><em> smallest element in the </em><code>m x n</code><em> multiplication table</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/05/02/multtable1-grid.jpg" style="width: 500px; height: 254px;" />
<pre>
<strong>Input:</strong> m = 3, n = 3, k = 5
<strong>Output:</strong> 3
<strong>Explanation:</strong> The 5<sup>th</sup> smallest number is 3.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/05/02/multtable2-grid.jpg" style="width: 493px; height: 293px;" />
<pre>
<strong>Input:</strong> m = 2, n = 3, k = 6
<strong>Output:</strong> 6
<strong>Explanation:</strong> The 6<sup>th</sup> smallest number is 6.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m, n &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= k &lt;= m * n</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn find_kth_number(black_m: i32, black_n: i32, black_k: i32) -> i32 {
        let (mut black_l, mut black_h) = (1, black_m * black_n);
        while black_l < black_h {
            let black_mid = black_l + (black_h - black_l) / 2;
            if (1..=black_m).map(|black_i| (black_mid / black_i).min(black_n)).sum::<i32>() < black_k {
                black_l = black_mid + 1;
            } else {
                black_h = black_mid;
            }
        }
        black_l
    }
}
```