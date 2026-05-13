# Generate Schedule

**Difficulty:** Medium
**Tags:** Array, Math, Greedy

---

## Problem

<p>You are given an integer <code>n</code> representing <code>n</code> teams. You are asked to generate a schedule such that:</p>

<ul>
	<li>Each team plays every other team <strong>exactly twice</strong>: once at home and once away.</li>
	<li>There is <strong>exactly one</strong> match per day; the schedule is a list of <strong>consecutive</strong> days and <code>schedule[i]</code> is the match on day <code>i</code>.</li>
	<li>No team plays on <strong>consecutive</strong> days.</li>
</ul>

<p>Return a 2D integer array <code>schedule</code>, where <code>schedule[i][0]</code> represents the home team and <code>schedule[i][1]</code> represents the away team. If multiple schedules meet the conditions, return <strong>any</strong> one of them.</p>

<p>If no schedule exists that meets the conditions, return an empty array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">[]</span></p>

<p><strong>Explanation:</strong></p>

<p>​​​​​​​Since each team plays every other team exactly twice, a total of 6 matches need to be played: <code>[0,1],[0,2],[1,2],[1,0],[2,0],[2,1]</code>.</p>

<p>It&#39;s not possible to create a schedule without at least one team playing consecutive days.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5</span></p>

<p><strong>Output:</strong> <span class="example-io">[[0,1],[2,3],[0,4],[1,2],[3,4],[0,2],[1,3],[2,4],[0,3],[1,4],[2,0],[3,1],[4,0],[2,1],[4,3],[1,0],[3,2],[4,1],[3,0],[4,2]]</span></p>

<p><strong>Explanation:</strong></p>

<p>Since each team plays every other team exactly twice, a total of 20 matches need to be played.</p>

<p>The output shows one of the schedules that meet the conditions. No team plays on consecutive days.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 50</code>​​​​​​​</li>
</ul>


## Hints

1. The problem can be solved greedily or using randomization.
2. Try pairing teams greedily while ensuring neither team played on the previous day.
3. Keep track of how many games each team still has to play.
4. Among teams that didn't play the previous day, match a pair whose combined remaining games is highest.
5. If a greedy choice leads to a dead end, try a different match order.

## Solution

```rust
impl Solution { pub fn generate_schedule(black_n: i32) -> Vec<Vec<i32>> { let black_n = black_n as usize; if black_n <= 4 { return vec![]; } let black_tot = black_n * (black_n - 1); let mut black_played = vec![0; black_n]; let mut black_m = vec![vec![false; black_n]; black_n]; let (mut black_l1, mut black_l2) = (-1i32, -1i32); let mut black_res = Vec::with_capacity(black_tot); for _ in 0..black_tot { let (mut black_min, mut black_f, mut black_s) = (i32::MAX, -1i32, -1i32); for black_i in 0..black_n { if black_i as i32 == black_l1 || black_i as i32 == black_l2 { continue; } for black_j in 0..black_n { if black_i == black_j || black_j as i32 == black_l1 || black_j as i32 == black_l2 || black_m[black_i][black_j] { continue; } let black_score = black_played[black_i] + black_played[black_j]; if black_score < black_min { black_min = black_score; black_f = black_i as i32; black_s = black_j as i32; } } } if black_f == -1 { return vec![]; } black_res.push(vec![black_f, black_s]); black_m[black_f as usize][black_s as usize] = true; black_played[black_f as usize] += 1; black_played[black_s as usize] += 1; black_l1 = black_f; black_l2 = black_s; } black_res } }
```