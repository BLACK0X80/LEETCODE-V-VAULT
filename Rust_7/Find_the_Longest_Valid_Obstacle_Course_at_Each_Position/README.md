# Find the Longest Valid Obstacle Course at Each Position

**Difficulty:** Hard
**Tags:** Array, Binary Search, Binary Indexed Tree

---

## Problem

<p>You want to build some obstacle courses. You are given a <strong>0-indexed</strong> integer array <code>obstacles</code> of length <code>n</code>, where <code>obstacles[i]</code> describes the height of the <code>i<sup>th</sup></code> obstacle.</p>

<p>For every index <code>i</code> between <code>0</code> and <code>n - 1</code> (<strong>inclusive</strong>), find the length of the <strong>longest obstacle course</strong> in <code>obstacles</code> such that:</p>

<ul>
	<li>You choose any number of obstacles between <code>0</code> and <code>i</code> <strong>inclusive</strong>.</li>
	<li>You must include the <code>i<sup>th</sup></code> obstacle in the course.</li>
	<li>You must put the chosen obstacles in the <strong>same order</strong> as they appear in <code>obstacles</code>.</li>
	<li>Every obstacle (except the first) is <strong>taller</strong> than or the <strong>same height</strong> as the obstacle immediately before it.</li>
</ul>

<p>Return <em>an array</em> <code>ans</code> <em>of length</em> <code>n</code>, <em>where</em> <code>ans[i]</code> <em>is the length of the <strong>longest obstacle course</strong> for index</em> <code>i</code><em> as described above</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> obstacles = [1,2,3,2]
<strong>Output:</strong> [1,2,3,3]
<strong>Explanation:</strong> The longest valid obstacle course at each position is:
- i = 0: [<u>1</u>], [1] has length 1.
- i = 1: [<u>1</u>,<u>2</u>], [1,2] has length 2.
- i = 2: [<u>1</u>,<u>2</u>,<u>3</u>], [1,2,3] has length 3.
- i = 3: [<u>1</u>,<u>2</u>,3,<u>2</u>], [1,2,2] has length 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> obstacles = [2,2,1]
<strong>Output:</strong> [1,2,1]
<strong>Explanation: </strong>The longest valid obstacle course at each position is:
- i = 0: [<u>2</u>], [2] has length 1.
- i = 1: [<u>2</u>,<u>2</u>], [2,2] has length 2.
- i = 2: [2,2,<u>1</u>], [1] has length 1.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> obstacles = [3,1,5,6,4,2]
<strong>Output:</strong> [1,1,2,3,2,2]
<strong>Explanation:</strong> The longest valid obstacle course at each position is:
- i = 0: [<u>3</u>], [3] has length 1.
- i = 1: [3,<u>1</u>], [1] has length 1.
- i = 2: [<u>3</u>,1,<u>5</u>], [3,5] has length 2. [1,5] is also valid.
- i = 3: [<u>3</u>,1,<u>5</u>,<u>6</u>], [3,5,6] has length 3. [1,5,6] is also valid.
- i = 4: [<u>3</u>,1,5,6,<u>4</u>], [3,4] has length 2. [1,4] is also valid.
- i = 5: [3,<u>1</u>,5,6,4,<u>2</u>], [1,2] has length 2.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == obstacles.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= obstacles[i] &lt;= 10<sup>7</sup></code></li>
</ul>


## Hints

1. Can you keep track of the minimum height for each obstacle course length?
2. You can use binary search to find the longest previous obstacle course length that satisfies the conditions.

## Solution

```rust
impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut black_sorted = obstacles.clone();
        black_sorted.sort_unstable();
        black_sorted.dedup();

        let mut black_bit = vec![0; black_sorted.len() + 1];
        let mut black_res = Vec::with_capacity(obstacles.len());

        fn black_update(black_bit: &mut Vec<i32>, mut black_idx: usize, black_val: i32) {
            while black_idx < black_bit.len() {
                black_bit[black_idx] = black_bit[black_idx].max(black_val);
                black_idx += (black_idx as i32 & -(black_idx as i32)) as usize;
            }
        }

        fn black_query(black_bit: &Vec<i32>, mut black_idx: usize) -> i32 {
            let mut black_max = 0;
            while black_idx > 0 {
                black_max = black_max.max(black_bit[black_idx]);
                black_idx -= (black_idx as i32 & -(black_idx as i32)) as usize;
            }
            black_max
        }

        for &black_h in &obstacles {
            let black_idx = black_sorted.binary_search(&black_h).unwrap() + 1;
            let black_cur = black_query(&black_bit, black_idx) + 1;
            black_res.push(black_cur);
            black_update(&mut black_bit, black_idx, black_cur);
        }
        black_res
    }
}
```