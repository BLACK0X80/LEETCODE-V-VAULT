# Maximum Points After Enemy Battles

**Difficulty:** Medium
**Tags:** Array, Greedy

---

## Problem

<p>You are given an integer array <code>enemyEnergies</code> denoting the energy values of various enemies.</p>

<p>You are also given an integer <code>currentEnergy</code> denoting the amount of energy you have initially.</p>

<p>You start with 0 points, and all the enemies are unmarked initially.</p>

<p>You can perform <strong>either</strong> of the following operations <strong>zero </strong>or multiple times to gain points:</p>

<ul>
	<li>Choose an <strong>unmarked</strong> enemy, <code>i</code>, such that <code>currentEnergy &gt;= enemyEnergies[i]</code>. By choosing this option:

	<ul>
		<li>You gain 1 point.</li>
		<li>Your energy is reduced by the enemy&#39;s energy, i.e. <code>currentEnergy = currentEnergy - enemyEnergies[i]</code>.</li>
	</ul>
	</li>
	<li>If you have <strong>at least</strong> 1 point, you can choose an <strong>unmarked</strong> enemy, <code>i</code>. By choosing this option:
	<ul>
		<li>Your energy increases by the enemy&#39;s energy, i.e. <code>currentEnergy = currentEnergy + enemyEnergies[i]</code>.</li>
		<li>The <font face="monospace">e</font>nemy <code>i</code> is <strong>marked</strong>.</li>
	</ul>
	</li>
</ul>

<p>Return an integer denoting the <strong>maximum</strong> points you can get in the end by optimally performing operations.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">enemyEnergies = [3,2,2], currentEnergy = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The following operations can be performed to get 3 points, which is the maximum:</p>

<ul>
	<li>First operation on enemy 1: <code>points</code> increases by 1, and <code>currentEnergy</code> decreases by 2. So, <code>points = 1</code>, and <code>currentEnergy = 0</code>.</li>
	<li>Second operation on enemy 0: <code>currentEnergy</code> increases by 3, and enemy 0 is marked. So, <code>points = 1</code>, <code>currentEnergy = 3</code>, and marked enemies = <code>[0]</code>.</li>
	<li>First operation on enemy 2: <code>points</code> increases by 1, and <code>currentEnergy</code> decreases by 2. So, <code>points = 2</code>, <code>currentEnergy = 1</code>, and marked enemies = <code>[0]</code>.</li>
	<li>Second operation on enemy 2: <code>currentEnergy</code> increases by 2, and enemy 2 is marked. So, <code>points = 2</code>, <code>currentEnergy = 3</code>, and marked enemies = <code>[0, 2]</code>.</li>
	<li>First operation on enemy 1: <code>points</code> increases by 1, and <code>currentEnergy</code> decreases by 2. So, <code>points = 3</code>, <code>currentEnergy = 1</code>, and marked enemies = <code>[0, 2]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">enemyEnergies = </span>[2]<span class="example-io">, currentEnergy = 10</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation: </strong></p>

<p>Performing the first operation 5 times on enemy 0 results in the maximum number of points.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= enemyEnergies.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= enemyEnergies[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= currentEnergy &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. The problem can be solved greedily.
2. Mark all the others except the smallest one first.
3. Use the smallest one to increase the energy.
4. Note that the initial energy should be no less than the smallest enemy.

## Solution

```rust
impl Solution { pub fn maximum_points(black_enemy_energies: Vec<i32>, black_current_energy: i32) -> i64 { let mut black_e = black_enemy_energies; black_e.sort(); if (black_current_energy as i64) < black_e[0] as i64 { return 0; } let mut black_total_energy = black_current_energy as i64; for black_i in 1..black_e.len() { black_total_energy += black_e[black_i] as i64; } black_total_energy / black_e[0] as i64 } }
```