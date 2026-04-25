# Largest Values From Labels

**Difficulty:** Medium
**Tags:** Array, Hash Table, Greedy, Sorting, Counting

---

## Problem

<p>You are given <code>n</code> item&#39;s value and label as two integer arrays <code>values</code> and <code>labels</code>. You are also given two integers <code>numWanted</code> and <code>useLimit</code>.</p>

<p>Your task is to find a subset of items with the <strong>maximum sum</strong> of their values such that:</p>

<ul>
	<li>The number of items is <strong>at most</strong> <code>numWanted</code>.</li>
	<li>The number of items with the same label is <strong>at most</strong> <code>useLimit</code>.</li>
</ul>

<p>Return the maximum sum.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">values = [5,4,3,2,1], labels = [1,1,2,2,3], numWanted = 3, useLimit = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">9</span></p>

<p><strong>Explanation:</strong></p>

<p>The subset chosen is the first, third, and fifth items with the sum of values 5 + 3 + 1.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">values = [5,4,3,2,1], labels = [1,3,3,3,2], numWanted = 3, useLimit = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">12</span></p>

<p><strong>Explanation:</strong></p>

<p>The subset chosen is the first, second, and third items with the sum of values 5 + 4 + 3.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">values = [9,8,8,7,6], labels = [0,0,0,1,1], numWanted = 3, useLimit = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">16</span></p>

<p><strong>Explanation:</strong></p>

<p>The subset chosen is the first and fourth items with the sum of values 9 + 7.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == values.length == labels.length</code></li>
	<li><code>1 &lt;= n &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= values[i], labels[i] &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= numWanted, useLimit &lt;= n</code></li>
</ul>


## Hints

1. Consider the items in order from largest to smallest value, and greedily take the items if they fall under the use_limit.  We can keep track of how many items of each label are used by using a hash table.

## Solution

```rust
impl Solution { pub fn largest_vals_from_labels(v: Vec<i32>, l: Vec<i32>, black_w: i32, black_lim: i32) -> i32 { let mut black_items: Vec<_> = v.into_iter().zip(l.into_iter()).collect(); black_items.sort_by_key(|k| -k.0); let (mut black_ans, mut black_cnt, mut black_map) = (0, 0, std::collections::HashMap::new()); for (val, label) in black_items { let black_e = black_map.entry(label).or_insert(0); if black_cnt < black_w && *black_e < black_lim { black_ans += val; *black_e += 1; black_cnt += 1; } } black_ans } }
```