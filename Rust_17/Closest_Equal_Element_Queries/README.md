# Closest Equal Element Queries

**Difficulty:** Medium
**Tags:** Array, Hash Table, Binary Search

---

## Problem

<p>You are given a <strong>circular</strong> array <code>nums</code> and an array <code>queries</code>.</p>

<p>For each query <code>i</code>, you have to find the following:</p>

<ul>
	<li>The <strong>minimum</strong> distance between the element at index <code>queries[i]</code> and <strong>any</strong> other index <code>j</code> in the <strong>circular</strong> array, where <code>nums[j] == nums[queries[i]]</code>. If no such index exists, the answer for that query should be -1.</li>
</ul>

<p>Return an array <code>answer</code> of the <strong>same</strong> size as <code>queries</code>, where <code>answer[i]</code> represents the result for query <code>i</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,3,1,4,1,3,2], queries = [0,3,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2,-1,3]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Query 0: The element at <code>queries[0] = 0</code> is <code>nums[0] = 1</code>. The nearest index with the same value is 2, and the distance between them is 2.</li>
	<li>Query 1: The element at <code>queries[1] = 3</code> is <code>nums[3] = 4</code>. No other index contains 4, so the result is -1.</li>
	<li>Query 2: The element at <code>queries[2] = 5</code> is <code>nums[5] = 3</code>. The nearest index with the same value is 1, and the distance between them is 3 (following the circular path: <code>5 -&gt; 6 -&gt; 0 -&gt; 1</code>).</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,4], queries = [0,1,2,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">[-1,-1,-1,-1]</span></p>

<p><strong>Explanation:</strong></p>

<p>Each value in <code>nums</code> is unique, so no index shares the same value as the queried element. This results in -1 for all queries.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= queries.length &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
	<li><code>0 &lt;= queries[i] &lt; nums.length</code></li>
</ul>


## Hints

1. Use a dictionary that maps each unique value in the array to a sorted list of its indices.
2. For each query, use binary search on the sorted indices list to find the nearest occurrences of the target value.

## Solution

```rust
impl Solution {
    pub fn solve_queries(black: Vec<i32>, black_black: Vec<i32>) -> Vec<i32> {
        { let mut black_m = std::collections::HashMap::new(); for (black_i, &black_x) in black.iter().enumerate() { black_m.entry(black_x).or_insert(vec![]).push(black_i as i32); } black_black.iter().map(|&black_b| { let black_p = &black_m[&black[black_b as usize]]; if black_p.len() < 2 { -1 } else { let black_n = black.len() as i32; let black_idx = black_p.binary_search(&(black_b as i32)).unwrap(); let black_l = black_p[(black_idx + black_p.len() - 1) % black_p.len()]; let black_r = black_p[(black_idx + 1) % black_p.len()]; std::cmp::min((black_b - black_l + black_n) % black_n, (black_r - black_b + black_n) % black_n) } }).collect() }
    }
}
```