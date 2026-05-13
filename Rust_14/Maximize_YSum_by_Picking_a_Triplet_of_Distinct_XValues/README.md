# Maximize Y‑Sum by Picking a Triplet of Distinct X‑Values

**Difficulty:** Medium
**Tags:** Array, Hash Table, Greedy, Sorting, Heap (Priority Queue)

---

## Problem

<p>You are given two integer arrays <code>x</code> and <code>y</code>, each of length <code>n</code>. You must choose three <strong>distinct</strong> indices <code>i</code>, <code>j</code>, and <code>k</code> such that:</p>

<ul>
	<li><code>x[i] != x[j]</code></li>
	<li><code>x[j] != x[k]</code></li>
	<li><code>x[k] != x[i]</code></li>
</ul>

<p>Your goal is to <strong>maximize</strong> the value of <code>y[i] + y[j] + y[k]</code> under these conditions. Return the <strong>maximum</strong> possible sum that can be obtained by choosing such a triplet of indices.</p>

<p>If no such triplet exists, return -1.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">x = [1,2,1,3,2], y = [5,3,4,6,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">14</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Choose <code>i = 0</code> (<code>x[i] = 1</code>, <code>y[i] = 5</code>), <code>j = 1</code> (<code>x[j] = 2</code>, <code>y[j] = 3</code>), <code>k = 3</code> (<code>x[k] = 3</code>, <code>y[k] = 6</code>).</li>
	<li>All three values chosen from <code>x</code> are distinct. <code>5 + 3 + 6 = 14</code> is the maximum we can obtain. Hence, the output is 14.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">x = [1,2,1,2], y = [4,5,6,7]</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>There are only two distinct values in <code>x</code>. Hence, the output is -1.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == x.length == y.length</code></li>
	<li><code>3 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= x[i], y[i] &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. For each unique <code>x</code>, keep only the maximum <code>y</code>; all other pairs with the same <code>x</code> are redundant.
2. Sort the pairs by <code>x</code> so that identical <code>x</code> values form contiguous blocks, then take the maximum <code>y</code> from each block.
3. Alternatively, use a map (or dictionary) from <code>x</code> to its largest <code>y</code>.

## Solution

```rust
impl Solution { pub fn max_sum_distinct_triplet(x: Vec<i32>, y: Vec<i32>) -> i32 { let mut black_m = std::collections::HashMap::new(); for (black_xi, black_yi) in x.iter().zip(y.iter()) { let black_e = black_m.entry(black_xi).or_insert(0); if *black_yi > *black_e { *black_e = *black_yi; } } if black_m.len() < 3 { -1 } else { let mut black_v: Vec<i32> = black_m.values().cloned().collect(); black_v.sort_unstable_by(|a, b| b.cmp(a)); black_v.iter().take(3).sum() } } }
```