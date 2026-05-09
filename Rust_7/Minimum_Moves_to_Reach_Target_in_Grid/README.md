# Minimum Moves to Reach Target in Grid

**Difficulty:** Hard
**Tags:** Math

---

## Problem

<p>You are given four integers <code>sx</code>, <code>sy</code>, <code>tx</code>, and <code>ty</code>, representing two points <code>(sx, sy)</code> and <code>(tx, ty)</code> on an infinitely large 2D grid.</p>

<p>You start at <code>(sx, sy)</code>.</p>

<p>At any point <code>(x, y)</code>, define <code>m = max(x, y)</code>. You can either:</p>

<ul>
	<li>Move to <code>(x + m, y)</code>, or</li>
	<li>Move to <code>(x, y + m)</code>.</li>
</ul>

<p>Return the <strong>minimum</strong> number of moves required to reach <code>(tx, ty)</code>. If it is impossible to reach the target, return -1.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">sx = 1, sy = 2, tx = 5, ty = 4</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The optimal path is:</p>

<ul>
	<li>Move 1: <code>max(1, 2) = 2</code>. Increase the y-coordinate by 2, moving from <code>(1, 2)</code> to <code>(1, 2 + 2) = (1, 4)</code>.</li>
	<li>Move 2: <code>max(1, 4) = 4</code>. Increase the x-coordinate by 4, moving from <code>(1, 4)</code> to <code>(1 + 4, 4) = (5, 4)</code>.</li>
</ul>

<p>Thus, the minimum number of moves to reach <code>(5, 4)</code> is 2.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">sx = 0, sy = 1, tx = 2, ty = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The optimal path is:</p>

<ul>
	<li>Move 1: <code>max(0, 1) = 1</code>. Increase the x-coordinate by 1, moving from <code>(0, 1)</code> to <code>(0 + 1, 1) = (1, 1)</code>.</li>
	<li>Move 2: <code>max(1, 1) = 1</code>. Increase the x-coordinate by 1, moving from <code>(1, 1)</code> to <code>(1 + 1, 1) = (2, 1)</code>.</li>
	<li>Move 3: <code>max(2, 1) = 2</code>. Increase the y-coordinate by 2, moving from <code>(2, 1)</code> to <code>(2, 1 + 2) = (2, 3)</code>.</li>
</ul>

<p>Thus, the minimum number of moves to reach <code>(2, 3)</code> is 3.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">sx = 1, sy = 1, tx = 2, ty = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>It is impossible to reach <code>(2, 2)</code> from <code>(1, 1)</code> using the allowed moves. Thus, the answer is -1.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= sx &lt;= tx &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= sy &lt;= ty &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Work backwards from <code>(tx, ty)</code> to <code>(sx, sy)</code>, undoing one move at each step.
2. If the larger coordinate >= 2 × (the smaller), undo by halving the larger; otherwise undo by subtracting the smaller from the larger.
3. Count these undo-steps until you hit <code>(sx, sy)</code> (return the count), or return -1 if you drop below or get stuck.

## Solution

```rust
impl Solution {
    pub fn min_moves(black_sx: i32, black_sy: i32, black_tx: i32, black_ty: i32) -> i32 {
        fn black_previ(black_x: i32, black_y: i32) -> Vec<(i32, i32)> {
            let mut black_can = Vec::new();
            if black_x == black_y {
                black_can.push((0, black_y));
                black_can.push((black_x, 0));
                return black_can;
            }
            if black_y < black_x {
                if black_x % 2 == 0 {
                    if black_x / 2 >= black_y {
                        black_can.push((black_x / 2, black_y));
                    } else if black_x - black_y <= black_y {
                        black_can.push((black_x - black_y, black_y));
                    }
                } else if black_x - black_y <= black_y {
                    black_can.push((black_x - black_y, black_y));
                }
            }
            if black_x < black_y {
                if black_y % 2 == 0 {
                    if black_y / 2 >= black_x {
                        black_can.push((black_x, black_y / 2));
                    } else if black_y - black_x <= black_x {
                        black_can.push((black_x, black_y - black_x));
                    }
                } else if black_y - black_x <= black_x {
                    black_can.push((black_x, black_y - black_x));
                }
            }
            black_can
        }

        fn black_dfs(black_sx: i32, black_sy: i32, black_a: i32, black_b: i32) -> i32 {
            if black_a < black_sx || black_b < black_sy {
                return 1_000_000_000;
            }
            if black_a == black_sx && black_b == black_sy {
                return 0;
            }
            let black_pre = black_previ(black_a, black_b);
            let mut black_ans = 1_000_000_000;
            for (black_x, black_y) in black_pre {
                black_ans = black_ans.min(1 + black_dfs(black_sx, black_sy, black_x, black_y));
            }
            black_ans
        }

        let black_res = black_dfs(black_sx, black_sy, black_tx, black_ty);
        if black_res >= 1_000_000_000 { -1 } else { black_res }
    }
}
```