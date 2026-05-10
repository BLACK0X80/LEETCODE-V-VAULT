# Maximum Capacity Within Budget

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Binary Search, Sorting

---

## Problem

<p>You are given two integer arrays <code>costs</code> and <code>capacity</code>, both of length <code>n</code>, where <code>costs[i]</code> represents the purchase cost of the <code>i<sup>th</sup></code> machine and <code>capacity[i]</code> represents its performance capacity.</p>

<p>You are also given an integer <code>budget</code>.</p>

<p>You may select <strong>at most two distinct</strong> machines such that the <strong>total cost</strong> of the selected machines is <strong>strictly less</strong> than <code>budget</code>.</p>

<p>Return the <strong>maximum</strong> achievable total capacity of the selected machines.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">costs = [4,8,5,3], capacity = [1,5,2,7], budget = 8</span></p>

<p><strong>Output:</strong> <span class="example-io">8</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Choose two machines with <code>costs[0] = 4</code> and <code>costs[3] = 3</code>.</li>
	<li>The total cost is <code>4 + 3 = 7</code>, which is strictly less than <code>budget = 8</code>.</li>
	<li>The maximum total capacity is <code>capacity[0] + capacity[3] = 1 + 7 = 8</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">costs = [3,5,7,4], capacity = [2,4,3,6], budget = 7</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Choose one machine with <code>costs[3] = 4</code>.</li>
	<li>The total cost is 4, which is strictly less than <code>budget = 7</code>.</li>
	<li>The maximum total capacity is <code>capacity[3] = 6</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">costs = [2,2,2], capacity = [3,5,4], budget = 5</span></p>

<p><strong>Output:</strong> <span class="example-io">9</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Choose two machines with <code>costs[1] = 2</code> and <code>costs[2] = 2</code>.</li>
	<li>The total cost is <code>2 + 2 = 4</code>, which is strictly less than <code>budget = 5</code>.</li>
	<li>The maximum total capacity is <code>capacity[1] + capacity[2] = 5 + 4 = 9</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == costs.length == capacity.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= costs[i], capacity[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= budget &lt;= 2 * 10<sup>5</sup></code></li>
</ul>


## Hints

1. Sort machines by increasing <code>costs</code>, keeping <code>capacity</code> aligned.
2. Build a prefix array where <code>prefMax[i]</code> stores the maximum <code>capacity</code> among machines with index <code><= i</code>.
3. For selecting one machine, take the maximum <code>capacity</code> among all machines with <code>cost < budget</code>.
4. For selecting two machines, fix machine <code>i</code> and binary search the largest index <code>j < i</code> with <code>costs[j] < budget - costs[i]</code>.
5. Update the answer with <code>capacity[i] + prefMax[j]</code> whenever such <code>j</code> exists.

## Solution

```rust
impl Solution { pub fn max_capacity(costs: Vec<i32>, capacity: Vec<i32>, budget: i32) -> i32 { let mut black_v: Vec<(i32, i32)> = costs.into_iter().zip(capacity.into_iter()).collect(); black_v.sort_unstable(); let black_n = black_v.len(); let mut black_p = vec![0; black_n]; let mut black_res = 0; for black_i in 0..black_n { black_p[black_i] = if black_i > 0 { black_p[black_i-1].max(black_v[black_i].1) } else { black_v[black_i].1 }; if black_v[black_i].0 < budget { black_res = black_res.max(black_v[black_i].1); } } for black_i in 0..black_n { let black_c = black_v[black_i].0; if black_c >= budget { break; } let black_lim = budget - black_c; let (mut black_l, mut black_r, mut black_j) = (0, black_i as i32 - 1, -1); while black_l <= black_r { let black_m = (black_l + black_r) / 2; if black_v[black_m as usize].0 < black_lim { black_j = black_m; black_l = black_m + 1; } else { black_r = black_m - 1; } } if black_j != -1 { black_res = black_res.max(black_v[black_i].1 + black_p[black_j as usize]); } } black_res } }
```