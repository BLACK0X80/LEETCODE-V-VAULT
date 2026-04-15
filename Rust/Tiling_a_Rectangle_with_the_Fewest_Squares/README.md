# Tiling a Rectangle with the Fewest Squares

**Difficulty:** Hard
**Tags:** Backtracking

---

## Problem

<p>Given a rectangle of size <code>n</code> x <code>m</code>, return <em>the minimum number of integer-sided squares that tile the rectangle</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2019/10/17/sample_11_1592.png" style="width: 154px; height: 106px;" /></p>

<pre>
<strong>Input:</strong> n = 2, m = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> <code>3</code> squares are necessary to cover the rectangle.
<code>2</code> (squares of <code>1x1</code>)
<code>1</code> (square of <code>2x2</code>)</pre>

<p><strong class="example">Example 2:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2019/10/17/sample_22_1592.png" style="width: 224px; height: 126px;" /></p>

<pre>
<strong>Input:</strong> n = 5, m = 8
<strong>Output:</strong> 5
</pre>

<p><strong class="example">Example 3:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2019/10/17/sample_33_1592.png" style="width: 224px; height: 189px;" /></p>

<pre>
<strong>Input:</strong> n = 11, m = 13
<strong>Output:</strong> 6
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n, m &lt;= 13</code></li>
</ul>


## Hints

1. Can you use backtracking to solve this problem ?.
2. Suppose you've placed a bunch of squares. Where is the natural spot to place the next square ?.
3. The maximum number of squares to be placed will be ≤ max(n,m).

## Solution

```rust
use std::cmp::min;

impl Solution {
    pub fn tiling_rectangle(black_n: i32, black_m: i32) -> i32 {
        let mut black_rect = vec![vec![false; black_m as usize]; black_n as usize];
        let mut black_ans = (black_n * black_m) as i32;

        fn black_solve(
            black_r: usize,
            black_c: usize,
            black_n: usize,
            black_m: usize,
            black_rect: &mut Vec<Vec<bool>>,
            black_count: i32,
            black_ans: &mut i32,
        ) {
            if black_count >= *black_ans {
                return;
            }

            let mut black_nr = black_r;
            let mut black_nc = black_c;
            let mut black_found = false;

            for i in black_r..black_n {
                for j in 0..black_m {
                    if !black_rect[i][j] {
                        black_nr = i;
                        black_nc = j;
                        black_found = true;
                        break;
                    }
                }
                if black_found { break; }
            }

            if !black_found {
                *black_ans = min(*black_ans, black_count);
                return;
            }

            let mut black_max_s = min(black_n - black_nr, black_m - black_nc);
            for black_s in (1..=black_max_s).rev() {
                if black_can_place(black_nr, black_nc, black_s, black_rect) {
                    black_place(black_nr, black_nc, black_s, black_rect, true);
                    black_solve(black_nr, black_nc, black_n, black_m, black_rect, black_count + 1, black_ans);
                    black_place(black_nr, black_nc, black_s, black_rect, false);
                }
            }
        }

        fn black_can_place(black_r: usize, black_c: usize, black_s: usize, black_rect: &Vec<Vec<bool>>) -> bool {
            for i in black_r..black_r + black_s {
                for j in black_c..black_c + black_s {
                    if black_rect[i][j] { return false; }
                }
            }
            true
        }

        fn black_place(black_r: usize, black_c: usize, black_s: usize, black_rect: &mut Vec<Vec<bool>>, black_val: bool) {
            for i in black_r..black_r + black_s {
                for j in black_c..black_c + black_s {
                    black_rect[i][j] = black_val;
                }
            }
        }

        black_solve(0, 0, black_n as usize, black_m as usize, &mut black_rect, 0, &mut black_ans);
        black_ans
    }
}
```