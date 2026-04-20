# Find the Kth Smallest Sum of a Matrix With Sorted Rows

**Difficulty:** Hard
**Tags:** Array, Binary Search, Heap (Priority Queue), Matrix

---

## Problem

<p>You are given an <code>m x n</code> matrix <code>mat</code> that has its rows sorted in non-decreasing order and an integer <code>k</code>.</p>

<p>You are allowed to choose <strong>exactly one element</strong> from each row to form an array.</p>

<p>Return <em>the </em><code>k<sup>th</sup></code><em> smallest array sum among all possible arrays</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> mat = [[1,3,11],[2,4,6]], k = 5
<strong>Output:</strong> 7
<strong>Explanation:</strong> Choosing one element from each row, the first k smallest sum are:
[1,2], [1,4], [3,2], [3,4], [1,6]. Where the 5th sum is 7.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> mat = [[1,3,11],[2,4,6]], k = 9
<strong>Output:</strong> 17
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> mat = [[1,10,10],[1,4,5],[2,3,6]], k = 7
<strong>Output:</strong> 9
<strong>Explanation:</strong> Choosing one element from each row, the first k smallest sum are:
[1,1,2], [1,1,3], [1,4,2], [1,4,3], [1,1,6], [1,5,2], [1,5,3]. Where the 7th sum is 9.  
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == mat.length</code></li>
	<li><code>n == mat.length[i]</code></li>
	<li><code>1 &lt;= m, n &lt;= 40</code></li>
	<li><code>1 &lt;= mat[i][j] &lt;= 5000</code></li>
	<li><code>1 &lt;= k &lt;= min(200, n<sup>m</sup>)</code></li>
	<li><code>mat[i]</code> is a non-decreasing array.</li>
</ul>


## Hints

1. Save all visited sums and corresponding indexes in a priority queue. Then, once you pop the smallest sum so far, you can quickly identify the next m candidates for smallest sum by incrementing each row index by 1.

## Solution

```rust
impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut sums = mat[0].clone();
        sums.truncate(k);
        for row in &mat[1..] {
            let mut next = Vec::with_capacity(sums.len() * row.len());
            for &s in &sums {
                for &x in row { next.push(s + x); }
            }
            next.sort_unstable();
            next.truncate(k);
            sums = next;
        }
        sums[k - 1]
    }
}
```