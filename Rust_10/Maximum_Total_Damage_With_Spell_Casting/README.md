# Maximum Total Damage With Spell Casting

**Difficulty:** Medium
**Tags:** Array, Hash Table, Two Pointers, Binary Search, Dynamic Programming, Sorting, Counting

---

## Problem

<p>A magician has various spells.</p>

<p>You are given an array <code>power</code>, where each element represents the damage of a spell. Multiple spells can have the same damage value.</p>

<p>It is a known fact that if a magician decides to cast a spell with a damage of <code>power[i]</code>, they <strong>cannot</strong> cast any spell with a damage of <code>power[i] - 2</code>, <code>power[i] - 1</code>, <code>power[i] + 1</code>, or <code>power[i] + 2</code>.</p>

<p>Each spell can be cast <strong>only once</strong>.</p>

<p>Return the <strong>maximum</strong> possible <em>total damage</em> that a magician can cast.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">power = [1,1,3,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>The maximum possible damage of 6 is produced by casting spells 0, 1, 3 with damage 1, 1, 4.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">power = [7,1,6,6]</span></p>

<p><strong>Output:</strong> <span class="example-io">13</span></p>

<p><strong>Explanation:</strong></p>

<p>The maximum possible damage of 13 is produced by casting spells 1, 2, 3 with damage 1, 6, 6.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= power.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= power[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. If we ever decide to use some spell with power <code>x</code>, then we will use all spells with power <code>x</code>.
2. Think of dynamic programming.
3. <code>dp[i][j]</code> represents the maximum damage considering up to the <code>i</code>-th unique spell and <code>j</code> represents the number of spells skipped (up to 3 as per constraints).

## Solution

```rust
impl Solution { pub fn maximum_total_damage(power: Vec<i32>) -> i64 { let mut black_map = std::collections::BTreeMap::new(); for x in power { *black_map.entry(x).or_insert(0i64) += 1; } let black_keys: Vec<i32> = black_map.keys().cloned().collect(); let mut black_dp = vec![0i64; black_keys.len() + 1]; for i in 0..black_keys.len() { let mut black_prev = i; while black_prev > 0 && black_keys[black_prev-1] >= black_keys[i] - 2 { black_prev -= 1; } black_dp[i+1] = black_dp[i].max(black_dp[black_prev] + black_keys[i] as i64 * black_map[&black_keys[i]]); } black_dp[black_keys.len()] } }
```