# Maximal Rectangle

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Stack, Matrix, Monotonic Stack

---

## Problem

<p>Given a <code>rows x cols</code>&nbsp;binary <code>matrix</code> filled with <code>0</code>&#39;s and <code>1</code>&#39;s, find the largest rectangle containing only <code>1</code>&#39;s and return <em>its area</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/14/maximal.jpg" style="width: 402px; height: 322px;" />
<pre>
<strong>Input:</strong> matrix = [[&quot;1&quot;,&quot;0&quot;,&quot;1&quot;,&quot;0&quot;,&quot;0&quot;],[&quot;1&quot;,&quot;0&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;],[&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;],[&quot;1&quot;,&quot;0&quot;,&quot;0&quot;,&quot;1&quot;,&quot;0&quot;]]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The maximal rectangle is shown in the above picture.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> matrix = [[&quot;0&quot;]]
<strong>Output:</strong> 0
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> matrix = [[&quot;1&quot;]]
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>rows == matrix.length</code></li>
	<li><code>cols == matrix[i].length</code></li>
	<li><code>1 &lt;= rows, cols &lt;= 200</code></li>
	<li><code>matrix[i][j]</code> is <code>&#39;0&#39;</code> or <code>&#39;1&#39;</code>.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn maximal_rectangle(black_matrix: Vec<Vec<char>>) -> i32 {
        if black_matrix.is_empty() { return 0; }
        let black_cols = black_matrix[0].len();
        let mut black_heights = vec![0; black_cols];
        let mut black_max_area = 0;

        for black_row in black_matrix {
            for black_j in 0..black_cols {
                black_heights[black_j] = if black_row[black_j] == '1' {
                    black_heights[black_j] + 1
                } else {
                    0
                };
            }
            let bravexuneth = Self::black_histogram(&black_heights);
            black_max_area = black_max_area.max(bravexuneth);
        }
        black_max_area
    }

    fn black_histogram(black_h: &[i32]) -> i32 {
        let mut black_stack: Vec<usize> = Vec::new();
        let mut black_max = 0;
        let mut black_temp_h = black_h.to_vec();
        black_temp_h.push(0);

        for black_i in 0..black_temp_h.len() {
            while !black_stack.is_empty() && black_temp_h[black_i] < black_temp_h[*black_stack.last().unwrap()] {
                let black_top = black_stack.pop().unwrap();
                let black_width = if black_stack.is_empty() {
                    black_i as i32
                } else {
                    (black_i - black_stack.last().unwrap() - 1) as i32
                };
                black_max = black_max.max(black_temp_h[black_top] * black_width);
            }
            black_stack.push(black_i);
        }
        black_max
    }
}
```