# Maximize the Minimum Game Score

**Difficulty:** Hard
**Tags:** Array, Binary Search, Greedy

---

## Problem

<p>You are given an array <code>points</code> of size <code>n</code> and an integer <code>m</code>. There is another array <code>gameScore</code> of size <code>n</code>, where <code>gameScore[i]</code> represents the score achieved at the <code>i<sup>th</sup></code> game. Initially, <code>gameScore[i] == 0</code> for all <code>i</code>.</p>

<p>You start at index -1, which is outside the array (before the first position at index 0). You can make <strong>at most</strong> <code>m</code> moves. In each move, you can either:</p>

<ul>
	<li>Increase the index by 1 and add <code>points[i]</code> to <code>gameScore[i]</code>.</li>
	<li>Decrease the index by 1 and add <code>points[i]</code> to <code>gameScore[i]</code>.</li>
</ul>

<p><strong>Note</strong> that the index must always remain within the bounds of the array after the first move.</p>

<p>Return the <strong>maximum possible minimum</strong> value in <code>gameScore</code> after <strong>at most</strong> <code>m</code> moves.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">points = [2,4], m = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>Initially, index <code>i = -1</code> and <code>gameScore = [0, 0]</code>.</p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Move</th>
			<th style="border: 1px solid black;">Index</th>
			<th style="border: 1px solid black;">gameScore</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">Increase <code>i</code></td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;"><code>[2, 0]</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">Increase <code>i</code></td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;"><code>[2, 4]</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">Decrease <code>i</code></td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;"><code>[4, 4]</code></td>
		</tr>
	</tbody>
</table>

<p>The minimum value in <code>gameScore</code> is 4, and this is the maximum possible minimum among all configurations. Hence, 4 is the output.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">points = [1,2,3], m = 5</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>Initially, index <code>i = -1</code> and <code>gameScore = [0, 0, 0]</code>.</p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Move</th>
			<th style="border: 1px solid black;">Index</th>
			<th style="border: 1px solid black;">gameScore</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">Increase <code>i</code></td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;"><code>[1, 0, 0]</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">Increase <code>i</code></td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;"><code>[1, 2, 0]</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">Decrease <code>i</code></td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;"><code>[2, 2, 0]</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">Increase <code>i</code></td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;"><code>[2, 4, 0]</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">Increase <code>i</code></td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;"><code>[2, 4, 3]</code></td>
		</tr>
	</tbody>
</table>

<p>The minimum value in <code>gameScore</code> is 2, and this is the maximum possible minimum among all configurations. Hence, 2 is the output.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n == points.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= points[i] &lt;= 10<sup>6</sup></code></li>
	<li><code>1 &lt;= m &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Can we use binary search?
2. What happens if you fix the game score as x?
3. We should go from i to (i + 1) back and forth, making the value for each index i (from left to right) no less than x.

## Solution

```rust
impl Solution {
    pub fn max_score(black_points: Vec<i32>, black_m_i32: i32) -> i64 {
        let black_m = black_m_i32 as i64;
        let mut black_l = 1i64;
        let mut black_r = 1_000_000_000_000_000i64;
        let mut black_result = 0i64;

        while black_l <= black_r {
            let black_mid = black_l + (black_r - black_l) / 2;
            if Self::black_possible(&black_points, black_m, black_mid) {
                black_result = black_mid;
                black_l = black_mid + 1;
            } else {
                black_r = black_mid - 1;
            }
        }
        black_result
    }

    fn black_possible(black_pts: &Vec<i32>, black_m: i64, black_target: i64) -> bool {
        let mut black_moves = 0i64;
        let mut black_advanced_moves = 0i64;
        let mut black_normal_moves = 0i64;

        for &black_game_point in black_pts {
            if black_moves > black_m {
                break;
            }
            let black_gp = black_game_point as i64;
            let mut black_games = (black_target + black_gp - 1) / black_gp;

            if black_advanced_moves >= black_games {
                black_advanced_moves = 0;
                black_normal_moves += 1;
            } else {
                let black_points_covered = black_advanced_moves * black_gp;
                black_games = (black_target - black_points_covered + black_gp - 1) / black_gp;
                black_moves += (2 * black_games) - 1;
                black_advanced_moves = (black_games - 1).max(0);
                black_moves += black_normal_moves;
                black_normal_moves = 0;
            }
        }
        black_moves <= black_m
    }
}
```