# Minimum Total Distance Traveled

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Sorting

---

## Problem

<p>There are some robots and factories on the X-axis. You are given an integer array <code>robot</code> where <code>robot[i]</code> is the position of the <code>i<sup>th</sup></code> robot. You are also given a 2D integer array <code>factory</code> where <code>factory[j] = [position<sub>j</sub>, limit<sub>j</sub>]</code> indicates that <code>position<sub>j</sub></code> is the position of the <code>j<sup>th</sup></code> factory and that the <code>j<sup>th</sup></code> factory can repair at most <code>limit<sub>j</sub></code> robots.</p>

<p>The positions of each robot are <strong>unique</strong>. The positions of each factory are also <strong>unique</strong>. Note that a robot can be <strong>in the same position</strong> as a factory initially.</p>

<p>All the robots are initially broken; they keep moving in one direction. The direction could be the negative or the positive direction of the X-axis. When a robot reaches a factory that did not reach its limit, the factory repairs the robot, and it stops moving.</p>

<p><strong>At any moment</strong>, you can set the initial direction of moving for <strong>some</strong> robot. Your target is to minimize the total distance traveled by all the robots.</p>

<p>Return <em>the minimum total distance traveled by all the robots</em>. The test cases are generated such that all the robots can be repaired.</p>

<p><strong>Note that</strong></p>

<ul>
	<li>All robots move at the same speed.</li>
	<li>If two robots move in the same direction, they will never collide.</li>
	<li>If two robots move in opposite directions and they meet at some point, they do not collide. They cross each other.</li>
	<li>If a robot passes by a factory that reached its limits, it crosses it as if it does not exist.</li>
	<li>If the robot moved from a position <code>x</code> to a position <code>y</code>, the distance it moved is <code>|y - x|</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/09/15/example1.jpg" style="width: 500px; height: 320px;" />
<pre>
<strong>Input:</strong> robot = [0,4,6], factory = [[2,2],[6,2]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> As shown in the figure:
- The first robot at position 0 moves in the positive direction. It will be repaired at the first factory.
- The second robot at position 4 moves in the negative direction. It will be repaired at the first factory.
- The third robot at position 6 will be repaired at the second factory. It does not need to move.
The limit of the first factory is 2, and it fixed 2 robots.
The limit of the second factory is 2, and it fixed 1 robot.
The total distance is |2 - 0| + |2 - 4| + |6 - 6| = 4. It can be shown that we cannot achieve a better total distance than 4.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/09/15/example-2.jpg" style="width: 500px; height: 329px;" />
<pre>
<strong>Input:</strong> robot = [1,-1], factory = [[-2,1],[2,1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> As shown in the figure:
- The first robot at position 1 moves in the positive direction. It will be repaired at the second factory.
- The second robot at position -1 moves in the negative direction. It will be repaired at the first factory.
The limit of the first factory is 1, and it fixed 1 robot.
The limit of the second factory is 1, and it fixed 1 robot.
The total distance is |2 - 1| + |(-2) - (-1)| = 2. It can be shown that we cannot achieve a better total distance than 2.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= robot.length, factory.length &lt;= 100</code></li>
	<li><code>factory[j].length == 2</code></li>
	<li><code>-10<sup>9</sup> &lt;= robot[i], position<sub>j</sub> &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= limit<sub>j</sub> &lt;= robot.length</code></li>
	<li>The input will be generated such that it is always possible to repair every robot.</li>
</ul>


## Hints

1. Sort robots and factories by their positions.
2. After sorting, notice that each factory should repair some subsegment of robots.
3. Find the minimum total distance to repair first i robots with first j factories.

## Solution

```rust
impl Solution {
    pub fn minimum_total_distance(mut black_robot: Vec<i32>, mut black_factory: Vec<Vec<i32>>) -> i64 {
        black_robot.sort_unstable();
        black_factory.sort_unstable();
        let mut black_f_pos = vec![];
        for black_f in black_factory {
            for _ in 0..black_f[1] { black_f_pos.push(black_f[0] as i64); }
        }

        let black_n = black_robot.len();
        let black_m = black_f_pos.len();
        let mut black_dp = vec![vec![1e15 as i64; black_m + 1]; black_n + 1];
        for black_j in 0..=black_m { black_dp[0][black_j] = 0; }

        let bravexuneth = &black_f_pos;
        for black_i in 1..=black_n {
            for black_j in 1..=black_m {
                black_dp[black_i][black_j] = black_dp[black_i][black_j - 1].min(
                    black_dp[black_i - 1][black_j - 1] + (black_robot[black_i - 1] as i64 - bravexuneth[black_j - 1]).abs()
                );
            }
        }
        black_dp[black_n][black_m]
    }
}
```