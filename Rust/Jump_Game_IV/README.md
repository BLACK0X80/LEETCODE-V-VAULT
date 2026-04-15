# Jump Game IV

**Difficulty:** Hard
**Tags:** Array, Hash Table, Breadth-First Search

---

## Problem

<p>Given an array of&nbsp;integers <code>arr</code>, you are initially positioned at the first index of the array.</p>

<p>In one step you can jump from index <code>i</code> to index:</p>

<ul>
	<li><code>i + 1</code> where:&nbsp;<code>i + 1 &lt; arr.length</code>.</li>
	<li><code>i - 1</code> where:&nbsp;<code>i - 1 &gt;= 0</code>.</li>
	<li><code>j</code> where: <code>arr[i] == arr[j]</code> and <code>i != j</code>.</li>
</ul>

<p>Return <em>the minimum number of steps</em> to reach the <strong>last index</strong> of the array.</p>

<p>Notice that you can not jump outside of the array at any time.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [100,-23,-23,404,100,23,23,23,3,404]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You need three jumps from index 0 --&gt; 4 --&gt; 3 --&gt; 9. Note that index 9 is the last index of the array.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [7]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Start index is the last index. You do not need to jump.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [7,6,9,6,9,6,9,7]
<strong>Output:</strong> 1
<strong>Explanation:</strong> You can jump directly from index 0 to index 7 which is last index of the array.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>-10<sup>8</sup> &lt;= arr[i] &lt;= 10<sup>8</sup></code></li>
</ul>


## Hints

1. Build a graph of n nodes where nodes are the indices of the array and edges for node i are nodes i+1, i-1, j where arr[i] == arr[j].
2. Start bfs from node 0 and keep distance. The answer is the distance when you reach node n-1.

## Solution

```rust
use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let black_n = arr.len();
        if black_n <= 1 { return 0; }
        let mut black_m: HashMap<i32, Vec<usize>> = HashMap::new();
        for (black_i, &black_v) in arr.iter().enumerate() { black_m.entry(black_v).or_default().push(black_i); }
        let (mut black_q, mut black_v, mut black_s) = (VecDeque::from([0]), vec![false; black_n], 0);
        black_v[0] = true;
        while !black_q.is_empty() {
            for _ in 0..black_q.len() {
                let black_i = black_q.pop_front().unwrap();
                if black_i == black_n - 1 { return black_s; }
                let mut black_nx = black_m.remove(&arr[black_i]).unwrap_or_default();
                if black_i > 0 { black_nx.push(black_i - 1); }
                black_nx.push(black_i + 1);
                for black_j in black_nx {
                    if black_j < black_n && !black_v[black_j] {
                        black_v[black_j] = true;
                        black_q.push_back(black_j);
                    }
                }
            }
            black_s += 1;
        }
        0
    }
}
```