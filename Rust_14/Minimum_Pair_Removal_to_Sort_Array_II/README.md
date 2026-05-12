# Minimum Pair Removal to Sort Array II

**Difficulty:** Hard
**Tags:** Array, Hash Table, Linked List, Heap (Priority Queue), Simulation, Doubly-Linked List, Ordered Set

---

## Problem

<p>Given an array <code>nums</code>, you can perform the following operation any number of times:</p>

<ul>
	<li>Select the <strong>adjacent</strong> pair with the <strong>minimum</strong> sum in <code>nums</code>. If multiple such pairs exist, choose the leftmost one.</li>
	<li>Replace the pair with their sum.</li>
</ul>

<p>Return the <strong>minimum number of operations</strong> needed to make the array <strong>non-decreasing</strong>.</p>

<p>An array is said to be <strong>non-decreasing</strong> if each element is greater than or equal to its previous element (if it exists).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [5,2,3,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The pair <code>(3,1)</code> has the minimum sum of 4. After replacement, <code>nums = [5,2,4]</code>.</li>
	<li>The pair <code>(2,4)</code> has the minimum sum of 6. After replacement, <code>nums = [5,6]</code>.</li>
</ul>

<p>The array <code>nums</code> became non-decreasing in two operations.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>The array <code>nums</code> is already sorted.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. We can perform the simulation using data structures.
2. Maintain an array index and value using a map since we need to find the next and previous ones.
3. Maintain the indices to be removed using a hash set.
4. Maintain the neighbor sums with the smaller indices (set or priority queue).
5. Keep the 3 structures in sync during the removals.

## Solution

```rust
use std::collections::BTreeSet;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 { return 0; }
        let mut vals: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        let mut nxt: Vec<usize> = (1..=n).collect();
        let mut prv: Vec<usize> = (0..n).map(|i| if i == 0 { 0 } else { i - 1 }).collect();
        let mut active = vec![true; n];

        let mut bad = 0i32;
        for i in 0..n-1 { if vals[i] > vals[i+1] { bad += 1; } }

        let mut heap: BTreeSet<(i64, usize)> = BTreeSet::new();
        for i in 0..n-1 { heap.insert((vals[i] + vals[i+1], i)); }

        let mut ops = 0;
        while bad > 0 {
            let &(_, i) = heap.iter().next().unwrap();
            let j = nxt[i];

            heap.remove(&(vals[i] + vals[j], i));

            let pi = prv[i];
            let nj = nxt[j];

            if i != pi && active[pi] {
                if vals[pi] > vals[i] { bad -= 1; }
                heap.remove(&(vals[pi] + vals[i], pi));
            }
            if vals[i] > vals[j] { bad -= 1; }
            if nj < n {
                if vals[j] > vals[nj] { bad -= 1; }
                heap.remove(&(vals[j] + vals[nj], j));
            }

            vals[i] += vals[j];
            active[j] = false;
            nxt[i] = nj;
            if nj < n { prv[nj] = i; }

            if i != pi && active[pi] {
                if vals[pi] > vals[i] { bad += 1; }
                heap.insert((vals[pi] + vals[i], pi));
            }
            if nj < n {
                if vals[i] > vals[nj] { bad += 1; }
                heap.insert((vals[i] + vals[nj], i));
            }

            ops += 1;
        }
        ops
    }
}
```