# Minimum Operations to Equalize Binary String

**Difficulty:** Hard
**Tags:** Math, String, Breadth-First Search, Union-Find, Ordered Set

---

## Problem

<p>You are given a binary string <code>s</code>, and an integer <code>k</code>.</p>

<p>In one operation, you must choose <strong>exactly</strong> <code>k</code> <strong>different</strong> indices and <strong>flip</strong> each <code>&#39;0&#39;</code> to <code>&#39;1&#39;</code> and each <code>&#39;1&#39;</code> to <code>&#39;0&#39;</code>.</p>

<p>Return the <strong>minimum</strong> number of operations required to make all characters in the string equal to <code>&#39;1&#39;</code>. If it is not possible, return -1.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;110&quot;, k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>There is one <code>&#39;0&#39;</code> in <code>s</code>.</li>
	<li>Since <code>k = 1</code>, we can flip it directly in one operation.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;0101&quot;, k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>One optimal set of operations choosing <code>k = 3</code> indices in each operation is:</p>

<ul>
	<li><strong>Operation 1</strong>: Flip indices <code>[0, 1, 3]</code>. <code>s</code> changes from <code>&quot;0101&quot;</code> to <code>&quot;1000&quot;</code>.</li>
	<li><strong>Operation 2</strong>: Flip indices <code>[1, 2, 3]</code>. <code>s</code> changes from <code>&quot;1000&quot;</code> to <code>&quot;1111&quot;</code>.</li>
</ul>

<p>Thus, the minimum number of operations is 2.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;101&quot;, k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p>Since <code>k = 2</code> and <code>s</code> has only one <code>&#39;0&#39;</code>, it is impossible to flip exactly <code>k</code> indices to make all <code>&#39;1&#39;</code>. Hence, the answer is -1.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>​​​​​​​5</sup></code></li>
	<li><code>s[i]</code> is either <code>&#39;0&#39;</code> or <code>&#39;1&#39;</code>.</li>
	<li><code>1 &lt;= k &lt;= s.length</code></li>
</ul>


## Hints

1. Model state as <code>z</code> = number of zeros; flipping <code>k</code> picks <code>i</code> zeros (<code>i</code> between <code>max(0, k - (n - z))</code> and <code>min(k, z)</code>) and transforms <code>z</code> to <code>z'</code> = <code>z + k - 2 * i</code>, so <code>z'</code> lies in a contiguous range and has parity <code>(z + k) % 2</code>.
2. Build a graph on states <code>0..n</code> and run <code>BFS</code> from initial <code>z</code> to reach <code>0</code>; each edge from <code>z</code> goes to all <code>z'</code> in that computed interval.
3. For speed, keep two ordered sets of unvisited states by parity and erase ranges with <code>lower_bound</code> while <code>BFSing</code> to achieve near <code>O(n log n)</code> time.

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn min_operations(black_s: String, black_k: i32) -> i32 {
        let black_n = black_s.len();
        let black_k = black_k as usize;
        let black_z = black_s.chars().filter(|&c| c == '0').count();

        if black_z == 0 { return 0; }

        let mut black_q = VecDeque::from([black_z]);
        let mut black_v = vec![-1; black_n + 1];
        black_v[black_z] = 0;

        while let Some(black_u) = black_q.pop_front() {
            let black_o = black_n - black_u;
            
            let black_min_i = if black_k > black_o { black_k - black_o } else { 0 };
            let black_max_i = black_u.min(black_k);
            
            let black_low = black_u + black_k - 2 * black_max_i;
            let black_high = black_u + black_k - 2 * black_min_i;

            let mut black_next = black_low;
            while black_next <= black_high {
                if black_v[black_next] == -1 {
                    black_v[black_next] = black_v[black_u] + 1;
                    if black_next == 0 { return black_v[black_next]; }
                    black_q.push_back(black_next);
                }
                black_next += 2;
            }
        }
        black_v[0]
    }
}
```