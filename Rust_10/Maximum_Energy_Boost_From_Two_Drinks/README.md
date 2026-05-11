# Maximum Energy Boost From Two Drinks

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given two integer arrays <code>energyDrinkA</code> and <code>energyDrinkB</code> of the same length <code>n</code> by a futuristic sports scientist. These arrays represent the energy boosts per hour provided by two different energy drinks, A and B, respectively.</p>

<p>You want to <em>maximize</em> your total energy boost by drinking one energy drink <em>per hour</em>. However, if you want to switch from consuming one energy drink to the other, you need to wait for <em>one hour</em> to cleanse your system (meaning you won&#39;t get any energy boost in that hour).</p>

<p>Return the <strong>maximum</strong> total energy boost you can gain in the next <code>n</code> hours.</p>

<p><strong>Note</strong> that you can start consuming <em>either</em> of the two energy drinks.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> energyDrinkA<span class="example-io"> = [1,3,1], </span>energyDrinkB<span class="example-io"> = [3,1,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p>To gain an energy boost of 5, drink only the energy drink A (or only B).</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> energyDrinkA<span class="example-io"> = [4,1,1], </span>energyDrinkB<span class="example-io"> = [1,1,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">7</span></p>

<p><strong>Explanation:</strong></p>

<p>To gain an energy boost of 7:</p>

<ul>
	<li>Drink the energy drink A for the first hour.</li>
	<li>Switch to the energy drink B and we lose the energy boost of the second hour.</li>
	<li>Gain the energy boost of the drink B in the third hour.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == energyDrinkA.length == energyDrinkB.length</code></li>
	<li><code>3 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= energyDrinkA[i], energyDrinkB[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Can we solve it using dynamic programming?
2. Define <code>dpA[i]</code> as the maximum energy boost if we consider only the first <code>i + 1</code> hours such that in the last hour, we drink the energy drink A.
3. Similarly define <code>dpB[i]</code>.
4. <code>dpA[i] = max(dpA[i - 1], dpB[i - 2]) + energyDrinkA[i]</code>
5. Similarly, fill <code>dpB</code>.
6. The answer is <code>max(dpA[n - 1], dpB[n - 1])</code>.

## Solution

```rust
impl Solution { pub fn max_energy_boost(a: Vec<i32>, b: Vec<i32>) -> i64 { let n = a.len(); let (mut black_a, mut black_b) = (vec![0i64; n + 1], vec![0i64; n + 1]); for i in 1..=n { black_a[i] = black_a[i-1] + a[i-1] as i64; black_b[i] = black_b[i-1] + b[i-1] as i64; if i >= 2 { black_a[i] = black_a[i].max(black_b[i-2] + a[i-1] as i64); black_b[i] = black_b[i].max(black_a[i-2] + b[i-1] as i64); } } black_a[n].max(black_b[n]) } }
```