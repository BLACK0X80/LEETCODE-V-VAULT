# Minimum Amount of Damage Dealt to Bob

**Difficulty:** Hard
**Tags:** Array, Greedy, Sorting

---

## Problem

<p>You are given an integer <code>power</code> and two integer arrays <code>damage</code> and <code>health</code>, both having length <code>n</code>.</p>

<p>Bob has <code>n</code> enemies, where enemy <code>i</code> will deal Bob <code>damage[i]</code> <strong>points</strong> of damage per second while they are <em>alive</em> (i.e. <code>health[i] &gt; 0</code>).</p>

<p>Every second, <strong>after</strong> the enemies deal damage to Bob, he chooses <strong>one</strong> of the enemies that is still <em>alive</em> and deals <code>power</code> points of damage to them.</p>

<p>Determine the <strong>minimum</strong> total amount of damage points that will be dealt to Bob before <strong>all</strong> <code>n</code> enemies are <em>dead</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">power = 4, damage = [1,2,3,4], health = [4,5,6,8]</span></p>

<p><strong>Output:</strong> <span class="example-io">39</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Attack enemy 3 in the first two seconds, after which enemy 3 will go down, the number of damage points dealt to Bob is <code>10 + 10 = 20</code> points.</li>
	<li>Attack enemy 2 in the next two seconds, after which enemy 2 will go down, the number of damage points dealt to Bob is <code>6 + 6 = 12</code> points.</li>
	<li>Attack enemy 0 in the next second, after which enemy 0 will go down, the number of damage points dealt to Bob is <code>3</code> points.</li>
	<li>Attack enemy 1 in the next two seconds, after which enemy 1 will go down, the number of damage points dealt to Bob is <code>2 + 2 = 4</code> points.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">power = 1, damage = [1,1,1,1], health = [1,2,3,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">20</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Attack enemy 0 in the first second, after which enemy 0 will go down, the number of damage points dealt to Bob is <code>4</code> points.</li>
	<li>Attack enemy 1 in the next two seconds, after which enemy 1 will go down, the number of damage points dealt to Bob is <code>3 + 3 = 6</code> points.</li>
	<li>Attack enemy 2 in the next three seconds, after which enemy 2 will go down, the number of damage points dealt to Bob is <code>2 + 2 + 2 = 6</code> points.</li>
	<li>Attack enemy 3 in the next four seconds, after which enemy 3 will go down, the number of damage points dealt to Bob is <code>1 + 1 + 1 + 1 = 4</code> points.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">power = 8, damage = [40], health = [59]</span></p>

<p><strong>Output:</strong> <span class="example-io">320</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= power &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= n == damage.length == health.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= damage[i], health[i] &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Can we use sorting here along with a custom comparator?
2. For any two enemies <code>i</code> and <code>j</code> with damages <code>damage[i]</code> and <code>damage[j]</code>, and time to take each of them down <code>t<sub>i</sub></code> and <code>t<sub>j</sub></code>, when is it better to choose enemy <code>i</code> over enemy <code>j</code> first?

## Solution

```rust
impl Solution {
    pub fn min_damage(black_power: i32, black_damage: Vec<i32>, black_health: Vec<i32>) -> i64 {
        let black_n = black_damage.len();
        let mut black_enemies: Vec<usize> = (0..black_n).collect();
        let black_p = black_power as i64;

        let black_get_time = |black_h: i32| -> i64 {
            (black_h as i64 + black_p - 1) / black_p
        };

        black_enemies.sort_by(|&black_a, &black_b| {
            let black_time_a = black_get_time(black_health[black_a]);
            let black_time_b = black_get_time(black_health[black_b]);
            let black_val_a = black_damage[black_a] as i64 * black_time_b;
            let black_val_b = black_damage[black_b] as i64 * black_time_a;
            black_val_b.cmp(&black_val_a)
        });

        let mut black_total_damage: i64 = 0;
        let mut black_current_sec: i64 = 0;
        let mut black_sum_damage: i64 = black_damage.iter().map(|&x| x as i64).sum();

        for &black_idx in &black_enemies {
            let black_t = black_get_time(black_health[black_idx]);
            black_total_damage += black_t * black_sum_damage;
            black_sum_damage -= black_damage[black_idx] as i64;
        }

        black_total_damage
    }
}
```