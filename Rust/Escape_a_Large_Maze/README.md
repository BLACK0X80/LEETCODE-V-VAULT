# Escape a Large Maze

**Difficulty:** Hard
**Tags:** Array, Hash Table, Depth-First Search, Breadth-First Search

---

## Problem

<p>There is a 1 million by 1 million grid on an XY-plane, and the coordinates of each grid square are <code>(x, y)</code>.</p>

<p>We start at the <code>source = [s<sub>x</sub>, s<sub>y</sub>]</code> square and want to reach the <code>target = [t<sub>x</sub>, t<sub>y</sub>]</code> square. There is also an array of <code>blocked</code> squares, where each <code>blocked[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> represents a blocked square with coordinates <code>(x<sub>i</sub>, y<sub>i</sub>)</code>.</p>

<p>Each move, we can walk one square north, east, south, or west if the square is <strong>not</strong> in the array of <code>blocked</code> squares. We are also not allowed to walk outside of the grid.</p>

<p>Return <code>true</code><em> if and only if it is possible to reach the </em><code>target</code><em> square from the </em><code>source</code><em> square through a sequence of valid moves</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> blocked = [[0,1],[1,0]], source = [0,0], target = [0,2]
<strong>Output:</strong> false
<strong>Explanation:</strong> The target square is inaccessible starting from the source square because we cannot move.
We cannot move north or east because those squares are blocked.
We cannot move south or west because we cannot go outside of the grid.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> blocked = [], source = [0,0], target = [999999,999999]
<strong>Output:</strong> true
<strong>Explanation:</strong> Because there are no blocked cells, it is possible to reach the target square.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= blocked.length &lt;= 200</code></li>
	<li><code>blocked[i].length == 2</code></li>
	<li><code>0 &lt;= x<sub>i</sub>, y<sub>i</sub> &lt; 10<sup>6</sup></code></li>
	<li><code>source.length == target.length == 2</code></li>
	<li><code>0 &lt;= s<sub>x</sub>, s<sub>y</sub>, t<sub>x</sub>, t<sub>y</sub> &lt; 10<sup>6</sup></code></li>
	<li><code>source != target</code></li>
	<li>It is guaranteed that <code>source</code> and <code>target</code> are not blocked.</li>
</ul>


## Hints

1. If we become stuck, there's either a loop around the source or around the target.
2. If there is a loop around say, the source, what is the maximum number of squares it can have?

## Solution

```rust
use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        let black_b: HashSet<(i32, i32)> = blocked.into_iter().map(|b| (b[0], b[1])).collect();
        let black_bfs = |black_s: &Vec<i32>, black_t: &Vec<i32>| -> bool {
            let (mut black_q, mut black_v) = (VecDeque::from([(black_s[0], black_s[1])]), HashSet::from([(black_s[0], black_s[1])]));
            while let Some((black_r, black_c)) = black_q.pop_front() {
                if black_r == black_t[0] && black_c == black_t[1] || black_v.len() > 20000 { return true; }
                for (black_dr, black_dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let (black_nr, black_nc) = (black_r + black_dr, black_c + black_dc);
                    if black_nr >= 0 && black_nr < 1000000 && black_nc >= 0 && black_nc < 1000000 && !black_b.contains(&(black_nr, black_nc)) && black_v.insert((black_nr, black_nc)) {
                        black_q.push_back((black_nr, black_nc));
                    }
                }
            }
            false
        };
        black_bfs(&source, &target) && black_bfs(&target, &source)
    }
}
```