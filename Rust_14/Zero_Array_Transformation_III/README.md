# Zero Array Transformation III

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Greedy, Sorting, Heap (Priority Queue), Prefix Sum

---

## Problem

<p>You are given an integer array <code>nums</code> of length <code>n</code> and a 2D array <code>queries</code> where <code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>]</code>.</p>

<p>Each <code>queries[i]</code> represents the following action on <code>nums</code>:</p>

<ul>
	<li>Decrement the value at each index in the range <code>[l<sub>i</sub>, r<sub>i</sub>]</code> in <code>nums</code> by <strong>at most</strong><strong> </strong>1.</li>
	<li>The amount by which the value is decremented can be chosen <strong>independently</strong> for each index.</li>
</ul>

<p>A <strong>Zero Array</strong> is an array with all its elements equal to 0.</p>

<p>Return the <strong>maximum </strong>number of elements that can be removed from <code>queries</code>, such that <code>nums</code> can still be converted to a <strong>zero array</strong> using the <em>remaining</em> queries. If it is not possible to convert <code>nums</code> to a <strong>zero array</strong>, return -1.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,0,2], queries = [[0,2],[0,2],[1,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>After removing <code>queries[2]</code>, <code>nums</code> can still be converted to a zero array.</p>

<ul>
	<li>Using <code>queries[0]</code>, decrement <code>nums[0]</code> and <code>nums[2]</code> by 1 and <code>nums[1]</code> by 0.</li>
	<li>Using <code>queries[1]</code>, decrement <code>nums[0]</code> and <code>nums[2]</code> by 1 and <code>nums[1]</code> by 0.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,1,1,1], queries = [[1,3],[0,2],[1,3],[1,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>We can remove <code>queries[2]</code> and <code>queries[3]</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,4], queries = [[0,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p><code>nums</code> cannot be converted to a zero array even after using all the queries.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>queries[i].length == 2</code></li>
	<li><code>0 &lt;= l<sub>i</sub> &lt;= r<sub>i</sub> &lt; nums.length</code></li>
</ul>


## Hints

1. Sort the queries.
2. We need to greedily pick the queries with farthest ending point first.

## Solution

```rust
impl Solution { pub fn max_removal(nums: Vec<i32>, mut black_q: Vec<Vec<i32>>) -> i32 { black_q.sort_unstable(); let (mut black_pq, mut black_i, mut black_c, mut black_res, mut black_d) = (std::collections::BinaryHeap::new(), 0, 0, 0, vec![0; nums.len() + 1]); for (idx, &v) in nums.iter().enumerate() { black_c += black_d[idx]; while black_i < black_q.len() && black_q[black_i][0] <= idx as i32 { black_pq.push(black_q[black_i][1]); black_i += 1; } while black_c < v { if let Some(r) = black_pq.pop() { if r < idx as i32 { return -1; } black_c += 1; black_res += 1; black_d[r as usize + 1] -= 1; } else { return -1; } } } (black_q.len() - black_res as usize) as i32 } }
```