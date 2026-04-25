# Distant Barcodes

**Difficulty:** Medium
**Tags:** Array, Hash Table, Greedy, Sorting, Heap (Priority Queue), Counting

---

## Problem

<p>In a warehouse, there is a row of barcodes, where the <code>i<sup>th</sup></code> barcode is <code>barcodes[i]</code>.</p>

<p>Rearrange the barcodes so that no two adjacent barcodes are equal. You may return any answer, and it is guaranteed an answer exists.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> barcodes = [1,1,1,2,2,2]
<strong>Output:</strong> [2,1,2,1,2,1]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> barcodes = [1,1,1,1,2,2,3,3]
<strong>Output:</strong> [1,3,1,3,1,2,1,2]
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= barcodes.length &lt;= 10000</code></li>
	<li><code>1 &lt;= barcodes[i] &lt;= 10000</code></li>
</ul>


## Hints

1. We want to always choose the most common or second most common element to write next.  What data structure allows us to query this effectively?

## Solution

```rust
impl Solution { pub fn rearrange_barcodes(black_b: Vec<i32>) -> Vec<i32> { let mut black_m = std::collections::HashMap::new(); for &x in &black_b { *black_m.entry(x).or_insert(0) += 1; } let mut black_v: Vec<_> = black_m.into_iter().collect(); black_v.sort_by_key(|k| -k.1); let (mut black_res, mut black_idx) = (vec![0; black_b.len()], 0); for (val, cnt) in black_v { for _ in 0..cnt { if black_idx >= black_b.len() { black_idx = 1; } black_res[black_idx] = val; black_idx += 2; } } black_res } }
```