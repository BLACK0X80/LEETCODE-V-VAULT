# Maximum Walls Destroyed by Robots

**Difficulty:** Hard
**Tags:** Array, Binary Search, Dynamic Programming, Sorting

---

## Problem

<div data-docx-has-block-data="false" data-lark-html-role="root" data-page-id="Rax8d6clvoFeVtx7bzXcvkVynwf">
<div class="old-record-id-Y5dGdSKIMoNTttxGhHLccrpEnaf">There is an endless straight line populated with some robots and walls. You are given integer arrays <code>robots</code>, <code>distance</code>, and <code>walls</code>:</div>
</div>

<ul>
	<li><code>robots[i]</code> is the position of the <code>i<sup>th</sup></code> robot.</li>
	<li><code>distance[i]</code> is the <strong>maximum</strong> distance the <code>i<sup>th</sup></code> robot&#39;s bullet can travel.</li>
	<li><code>walls[j]</code> is the position of the <code>j<sup>th</sup></code> wall.</li>
</ul>

<p>Every robot has <strong>one</strong> bullet that can either fire to the left or the right <strong>at most </strong><code>distance[i]</code> meters.</p>

<p>A bullet destroys every wall in its path that lies within its range. Robots are fixed obstacles: if a bullet hits another robot before reaching a wall, it <strong>immediately stops</strong> at that robot and cannot continue.</p>

<p>Return the <strong>maximum</strong> number of <strong>unique</strong> walls that can be destroyed by the robots.</p>

<p>Notes:</p>

<ul>
	<li>A wall and a robot may share the same position; the wall can be destroyed by the robot at that position.</li>
	<li>Robots are not destroyed by bullets.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">robots = [4], distance = [3], walls = [1,10]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li><code>robots[0] = 4</code> fires <strong>left</strong> with <code>distance[0] = 3</code>, covering <code>[1, 4]</code> and destroys <code>walls[0] = 1</code>.</li>
	<li>Thus, the answer is 1.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">robots = [10,2], distance = [5,1], walls = [5,2,7]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li><code>robots[0] = 10</code> fires <strong>left</strong> with <code>distance[0] = 5</code>, covering <code>[5, 10]</code> and destroys <code>walls[0] = 5</code> and <code>walls[2] = 7</code>.</li>
	<li><code>robots[1] = 2</code> fires <strong>left</strong> with <code>distance[1] = 1</code>, covering <code>[1, 2]</code> and destroys <code>walls[1] = 2</code>.</li>
	<li>Thus, the answer is 3.</li>
</ul>
</div>
<strong class="example">Example 3:</strong>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">robots = [1,2], distance = [100,1], walls = [10]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>In this example, only <code>robots[0]</code> can reach the wall, but its shot to the <strong>right</strong> is blocked by <code>robots[1]</code>; thus the answer is 0.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= robots.length == distance.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= walls.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= robots[i], walls[j] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= distance[i] &lt;= 10<sup>5</sup></code></li>
	<li>All values in <code>robots</code> are <strong>unique</strong></li>
	<li>All values in <code>walls</code> are <strong>unique</strong></li>
</ul>


## Hints

1. Sort both the robots and walls arrays. This will help in efficiently processing positions and performing range queries.
2. Each robot can shoot either left or right. However, if a robot fires and another robot is in its path, the bullet stops. You need to use the positions of neighboring robots to limit the shooting range.
3. Use binary search (lower_bound and upper_bound) to count how many walls fall within a certain range.
4. You can use dynamic programming to keep track of the maximum number of walls destroyed so far, depending on the direction the previous robot shot.

## Solution

```rust
impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let n = robots.len();
        let mut robots_dist: Vec<(i32, i32)> = robots.iter().copied().zip(distance.iter().copied()).collect();
        robots_dist.sort();
        let mut walls = walls;
        walls.sort();

        let lb = |v: &[i32], x: i32| v.partition_point(|&a| a < x);
        let ub = |v: &[i32], x: i32| v.partition_point(|&a| a <= x);

        let mut left = vec![0i32; n];
        let mut right = vec![0i32; n];
        let mut num = vec![0i32; n];

        for i in 0..n {
            let (pos, dist) = robots_dist[i];

            let left_bound = if i > 0 { (pos - dist).max(robots_dist[i-1].0 + 1) } else { pos - dist };
            let right_bound = if i+1 < n { (pos + dist).min(robots_dist[i+1].0 - 1) } else { pos + dist };

            left[i]  = (ub(&walls, pos) - lb(&walls, left_bound)) as i32;
            right[i] = (ub(&walls, right_bound) - lb(&walls, pos)) as i32;

            if i > 0 {
                num[i] = (ub(&walls, pos) - lb(&walls, robots_dist[i-1].0)) as i32;
            }
        }

        let (mut sub_left, mut sub_right) = (left[0], right[0]);
        for i in 1..n {
            let cur_left  = (sub_left + left[i]).max(sub_right - right[i-1] + (left[i] + right[i-1]).min(num[i]));
            let cur_right = (sub_left + right[i]).max(sub_right + right[i]);
            sub_left  = cur_left;
            sub_right = cur_right;
        }

        sub_left.max(sub_right)
    }
}
```