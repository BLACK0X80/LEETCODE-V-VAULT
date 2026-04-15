# Minimum Interval to Include Each Query

**Difficulty:** Hard
**Tags:** Array, Binary Search, Sweep Line, Sorting, Heap (Priority Queue)

---

## Problem

<p>You are given a 2D integer array <code>intervals</code>, where <code>intervals[i] = [left<sub>i</sub>, right<sub>i</sub>]</code> describes the <code>i<sup>th</sup></code> interval starting at <code>left<sub>i</sub></code> and ending at <code>right<sub>i</sub></code> <strong>(inclusive)</strong>. The <strong>size</strong> of an interval is defined as the number of integers it contains, or more formally <code>right<sub>i</sub> - left<sub>i</sub> + 1</code>.</p>

<p>You are also given an integer array <code>queries</code>. The answer to the <code>j<sup>th</sup></code> query is the <strong>size of the smallest interval</strong> <code>i</code> such that <code>left<sub>i</sub> &lt;= queries[j] &lt;= right<sub>i</sub></code>. If no such interval exists, the answer is <code>-1</code>.</p>

<p>Return <em>an array containing the answers to the queries</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> intervals = [[1,4],[2,4],[3,6],[4,4]], queries = [2,3,4,5]
<strong>Output:</strong> [3,3,1,4]
<strong>Explanation:</strong> The queries are processed as follows:
- Query = 2: The interval [2,4] is the smallest interval containing 2. The answer is 4 - 2 + 1 = 3.
- Query = 3: The interval [2,4] is the smallest interval containing 3. The answer is 4 - 2 + 1 = 3.
- Query = 4: The interval [4,4] is the smallest interval containing 4. The answer is 4 - 4 + 1 = 1.
- Query = 5: The interval [3,6] is the smallest interval containing 5. The answer is 6 - 3 + 1 = 4.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> intervals = [[2,3],[2,5],[1,8],[20,25]], queries = [2,19,5,22]
<strong>Output:</strong> [2,-1,4,6]
<strong>Explanation:</strong> The queries are processed as follows:
- Query = 2: The interval [2,3] is the smallest interval containing 2. The answer is 3 - 2 + 1 = 2.
- Query = 19: None of the intervals contain 19. The answer is -1.
- Query = 5: The interval [2,5] is the smallest interval containing 5. The answer is 5 - 2 + 1 = 4.
- Query = 22: The interval [20,25] is the smallest interval containing 22. The answer is 25 - 20 + 1 = 6.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= intervals.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>intervals[i].length == 2</code></li>
	<li><code>1 &lt;= left<sub>i</sub> &lt;= right<sub>i</sub> &lt;= 10<sup>7</sup></code></li>
	<li><code>1 &lt;= queries[j] &lt;= 10<sup>7</sup></code></li>
</ul>


## Hints

1. Is there a way to order the intervals and queries such that it takes less time to query?
2. Is there a way to add and remove intervals by going from the smallest query to the largest query to find the minimum size?

## Solution

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_interval(mut black1: Vec<Vec<i32>>, black2: Vec<i32>) -> Vec<i32> {
        black1.sort_unstable_by_key(|i| i[0]);
        let mut black3: Vec<(i32, usize)> = black2.into_iter().enumerate().map(|(i, q)| (q, i)).collect();
        black3.sort_unstable();
        
        let mut black4 = vec![-1; black3.len()];
        let mut black5 = BinaryHeap::new();
        let mut black6 = 0;
        
        for (q, idx) in black3 {
            while black6 < black1.len() && black1[black6][0] <= q {
                let black7 = black1[black6][1] - black1[black6][0] + 1;
                black5.push(Reverse((black7, black1[black6][1])));
                black6 += 1;
            }
            while let Some(Reverse((_, end))) = black5.peek() {
                if *end < q { black5.pop(); } else { break; }
            }
            if let Some(Reverse((size, _))) = black5.peek() {
                black4[idx] = *size;
            }
        }
        black4
    }
}
```