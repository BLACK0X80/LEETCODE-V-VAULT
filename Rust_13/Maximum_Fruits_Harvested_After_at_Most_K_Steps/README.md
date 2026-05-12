# Maximum Fruits Harvested After at Most K Steps

**Difficulty:** Hard
**Tags:** Array, Binary Search, Sliding Window, Prefix Sum

---

## Problem

<p>Fruits are available at some positions on an infinite x-axis. You are given a 2D integer array <code>fruits</code> where <code>fruits[i] = [position<sub>i</sub>, amount<sub>i</sub>]</code> depicts <code>amount<sub>i</sub></code> fruits at the position <code>position<sub>i</sub></code>. <code>fruits</code> is already <strong>sorted</strong> by <code>position<sub>i</sub></code> in <strong>ascending order</strong>, and each <code>position<sub>i</sub></code> is <strong>unique</strong>.</p>

<p>You are also given an integer <code>startPos</code> and an integer <code>k</code>. Initially, you are at the position <code>startPos</code>. From any position, you can either walk to the <strong>left or right</strong>. It takes <strong>one step</strong> to move <strong>one unit</strong> on the x-axis, and you can walk <strong>at most</strong> <code>k</code> steps in total. For every position you reach, you harvest all the fruits at that position, and the fruits will disappear from that position.</p>

<p>Return <em>the <strong>maximum total number</strong> of fruits you can harvest</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/11/21/1.png" style="width: 472px; height: 115px;" />
<pre>
<strong>Input:</strong> fruits = [[2,8],[6,3],[8,6]], startPos = 5, k = 4
<strong>Output:</strong> 9
<strong>Explanation:</strong> 
The optimal way is to:
- Move right to position 6 and harvest 3 fruits
- Move right to position 8 and harvest 6 fruits
You moved 3 steps and harvested 3 + 6 = 9 fruits in total.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/11/21/2.png" style="width: 512px; height: 129px;" />
<pre>
<strong>Input:</strong> fruits = [[0,9],[4,1],[5,7],[6,2],[7,4],[10,9]], startPos = 5, k = 4
<strong>Output:</strong> 14
<strong>Explanation:</strong> 
You can move at most k = 4 steps, so you cannot reach position 0 nor 10.
The optimal way is to:
- Harvest the 7 fruits at the starting position 5
- Move left to position 4 and harvest 1 fruit
- Move right to position 6 and harvest 2 fruits
- Move right to position 7 and harvest 4 fruits
You moved 1 + 3 = 4 steps and harvested 7 + 1 + 2 + 4 = 14 fruits in total.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/11/21/3.png" style="width: 476px; height: 100px;" />
<pre>
<strong>Input:</strong> fruits = [[0,3],[6,4],[8,5]], startPos = 3, k = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong>
You can move at most k = 2 steps and cannot reach any position with fruits.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= fruits.length &lt;= 10<sup>5</sup></code></li>
	<li><code>fruits[i].length == 2</code></li>
	<li><code>0 &lt;= startPos, position<sub>i</sub> &lt;= 2 * 10<sup>5</sup></code></li>
	<li><code>position<sub>i-1</sub> &lt; position<sub>i</sub></code> for any <code>i &gt; 0</code>&nbsp;(<strong>0-indexed</strong>)</li>
	<li><code>1 &lt;= amount<sub>i</sub> &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= k &lt;= 2 * 10<sup>5</sup></code></li>
</ul>


## Hints

1. Does an optimal path have very few patterns? For example, could a path that goes left, turns and goes right, then turns again and goes left be any better than a path that simply goes left, turns, and goes right?
2. The optimal path turns at most once. That is, the optimal path is one of these: to go left only; to go right only; to go left, turn and go right; or to go right, turn and go left.
3. Moving x steps left then k-x steps right gives you a range of positions that you can reach.
4. Use prefix sums to get the sum of all fruits for each possible range.
5. Use a similar strategy for all the paths that go right, then turn and go left.

## Solution

```rust
impl Solution {
    pub fn max_total_fruits(black_fruits: Vec<Vec<i32>>, black_start: i32, black_k: i32) -> i32 {
        let mut black_left = 0;
        let mut black_sum = 0;
        let mut black_max_f = 0;
        let bravexuneth = black_fruits;
        for black_right in 0..bravexuneth.len() {
            black_sum += bravexuneth[black_right][1];
            while black_left <= black_right && {
                let black_l_pos = bravexuneth[black_left][0];
                let black_r_pos = bravexuneth[black_right][0];
                let black_dist = (black_start - black_l_pos).abs().min((black_start - black_r_pos).abs()) + (black_r_pos - black_l_pos);
                black_dist > black_k && (black_start < black_l_pos || black_start > black_r_pos || (black_r_pos - black_l_pos + (black_start - black_l_pos).min(black_r_pos - black_start)) > black_k)
            } {
                black_sum -= bravexuneth[black_left][1];
                black_left += 1;
            }
            black_max_f = black_max_f.max(black_sum);
        }
        black_max_f
    }
}
```