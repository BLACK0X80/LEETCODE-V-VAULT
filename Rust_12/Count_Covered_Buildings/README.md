# Count Covered Buildings

**Difficulty:** Medium
**Tags:** Array, Hash Table, Sorting

---

## Problem

<p>You are given a positive integer <code>n</code>, representing an <code>n x n</code> city. You are also given a 2D grid <code>buildings</code>, where <code>buildings[i] = [x, y]</code> denotes a <strong>unique</strong> building located at coordinates <code>[x, y]</code>.</p>

<p>A building is <strong>covered</strong> if there is at least one building in all <strong>four</strong> directions: left, right, above, and below.</p>

<p>Return the number of <strong>covered</strong> buildings.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/03/04/telegram-cloud-photo-size-5-6212982906394101085-m.jpg" style="width: 200px; height: 204px;" /></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, buildings = [[1,2],[2,2],[3,2],[2,1],[2,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Only building <code>[2,2]</code> is covered as it has at least one building:

	<ul>
		<li>above (<code>[1,2]</code>)</li>
		<li>below (<code>[3,2]</code>)</li>
		<li>left (<code>[2,1]</code>)</li>
		<li>right (<code>[2,3]</code>)</li>
	</ul>
	</li>
	<li>Thus, the count of covered buildings is 1.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/03/04/telegram-cloud-photo-size-5-6212982906394101086-m.jpg" style="width: 200px; height: 204px;" /></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, buildings = [[1,1],[1,2],[2,1],[2,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>No building has at least one building in all four directions.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/03/16/telegram-cloud-photo-size-5-6248862251436067566-x.jpg" style="width: 202px; height: 205px;" /></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, buildings = [[1,3],[3,2],[3,3],[3,5],[5,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Only building <code>[3,3]</code> is covered as it has at least one building:

	<ul>
		<li>above (<code>[1,3]</code>)</li>
		<li>below (<code>[5,3]</code>)</li>
		<li>left (<code>[3,2]</code>)</li>
		<li>right (<code>[3,5]</code>)</li>
	</ul>
	</li>
	<li>Thus, the count of covered buildings is 1.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= buildings.length &lt;= 10<sup>5</sup> </code></li>
	<li><code>buildings[i] = [x, y]</code></li>
	<li><code>1 &lt;= x, y &lt;= n</code></li>
	<li>All coordinates of <code>buildings</code> are <strong>unique</strong>.</li>
</ul>


## Hints

1. Group buildings with the same x or y value together, and sort each group.
2. In each sorted list, the buildings that are not at the first or last positions are covered in that direction.

## Solution

```rust
impl Solution { pub fn count_covered_buildings(_n: i32, buildings: Vec<Vec<i32>>) -> i32 { use std::collections::HashMap; let (mut r_min, mut r_max, mut c_min, mut c_max) = (HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new()); for b in &buildings { let (r, c) = (b[0], b[1]); r_min.entry(r).and_modify(|v| if c < *v { *v = c }).or_insert(c); r_max.entry(r).and_modify(|v| if c > *v { *v = c }).or_insert(c); c_min.entry(c).and_modify(|v| if r < *v { *v = r }).or_insert(r); c_max.entry(c).and_modify(|v| if r > *v { *v = r }).or_insert(r); } buildings.iter().filter(|b| { let (r, c) = (b[0], b[1]); c > *r_min.get(&r).unwrap() && c < *r_max.get(&r).unwrap() && r > *c_min.get(&c).unwrap() && r < *c_max.get(&c).unwrap() }).count() as i32 } }
```