# Unit Conversion I

**Difficulty:** Medium
**Tags:** Depth-First Search, Breadth-First Search, Graph Theory

---

## Problem

<p>There are <code>n</code> types of units indexed from <code>0</code> to <code>n - 1</code>. You are given a 2D integer array <code>conversions</code> of length <code>n - 1</code>, where <code>conversions[i] = [sourceUnit<sub>i</sub>, targetUnit<sub>i</sub>, conversionFactor<sub>i</sub>]</code>. This indicates that a single unit of type <code>sourceUnit<sub>i</sub></code> is equivalent to <code>conversionFactor<sub>i</sub></code> units of type <code>targetUnit<sub>i</sub></code>.</p>

<p>Return an array <code>baseUnitConversion</code> of length <code>n</code>, where <code>baseUnitConversion[i]</code> is the number of units of type <code>i</code> equivalent to a single unit of type 0. Since the answer may be large, return each <code>baseUnitConversion[i]</code> <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">conversions = [[0,1,2],[1,2,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,2,6]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Convert a single unit of type 0 into 2 units of type 1 using <code>conversions[0]</code>.</li>
	<li>Convert a single unit of type 0 into 6 units of type 2 using <code>conversions[0]</code>, then <code>conversions[1]</code>.</li>
</ul>
<img alt="" src="https://assets.leetcode.com/uploads/2025/03/12/example1.png" style="width: 545px; height: 118px;" /></div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">conversions = [[0,1,2],[0,2,3],[1,3,4],[1,4,5],[2,5,2],[4,6,3],[5,7,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,2,3,8,10,6,30,24]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Convert a single unit of type 0 into 2 units of type 1 using <code>conversions[0]</code>.</li>
	<li>Convert a single unit of type 0 into 3 units of type 2 using <code>conversions[1]</code>.</li>
	<li>Convert a single unit of type 0 into 8 units of type 3 using <code>conversions[0]</code>, then <code>conversions[2]</code>.</li>
	<li>Convert a single unit of type 0 into 10 units of type 4 using <code>conversions[0]</code>, then <code>conversions[3]</code>.</li>
	<li>Convert a single unit of type 0 into 6 units of type 5 using <code>conversions[1]</code>, then <code>conversions[4]</code>.</li>
	<li>Convert a single unit of type 0 into 30 units of type 6 using <code>conversions[0]</code>, <code>conversions[3]</code>, then <code>conversions[5]</code>.</li>
	<li>Convert a single unit of type 0 into 24 units of type 7 using <code>conversions[1]</code>, <code>conversions[4]</code>, then <code>conversions[6]</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>conversions.length == n - 1</code></li>
	<li><code>0 &lt;= sourceUnit<sub>i</sub>, targetUnit<sub>i</sub> &lt; n</code></li>
	<li><code>1 &lt;= conversionFactor<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
	<li>It is guaranteed that unit 0 can be converted into any other unit through a <strong>unique</strong> combination of conversions without using any conversions in the opposite direction.</li>
</ul>


## Hints

1. The input is a weighted directed tree rooted at 0.
2. Launch a BFS from node 0 and multiply the weights on the path.

## Solution

```rust
impl Solution { pub fn base_unit_conversions(black_c: Vec<Vec<i32>>) -> Vec<i32> { let black_n = black_c.len() + 1; let mut black_adj = vec![vec![]; black_n]; for black_con in black_c { black_adj[black_con[0] as usize].push((black_con[1] as usize, black_con[2] as i64)); } let (mut black_res, mut black_q) = (vec![0i64; black_n], std::collections::VecDeque::from([(0, 1i64)])); black_res[0] = 1; while let Some((black_u, black_f)) = black_q.pop_front() { for &(black_v, black_w) in &black_adj[black_u] { black_res[black_v] = (black_f * black_w) % 1_000_000_007; black_q.push_back((black_v, black_res[black_v])); } } black_res.into_iter().map(|black_x| black_x as i32).collect() } }
```