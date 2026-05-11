# Minimum Time to Complete All Deliveries

**Difficulty:** Medium
**Tags:** Math, Binary Search

---

## Problem

<p>You are given two integer arrays of size 2: <code>d = [d<sub>1</sub>, d<sub>2</sub>]</code> and <code>r = [r<sub>1</sub>, r<sub>2</sub>]</code>.</p>

<p>Two delivery drones are tasked with completing a specific number of deliveries. Drone <code>i</code> must complete <code>d<sub>i</sub></code> deliveries.</p>

<p>Each delivery takes <strong>exactly</strong> one hour and <strong>only one</strong> drone can make a delivery at any given hour.</p>

<p>Additionally, both drones require recharging at specific intervals during which they cannot make deliveries. Drone <code>i</code> must recharge every <code>r<sub>i</sub></code> hours (i.e. at hours that are multiples of <code>r<sub>i</sub></code>).</p>

<p>Return an integer denoting the <strong>minimum</strong> total time (in hours) required to complete all deliveries.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">d = [3,1], r = [2,3]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The first drone delivers at hours 1, 3, 5 (recharges at hours 2, 4).</li>
	<li>The second drone delivers at hour 2 (recharges at hour 3).</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">d = [1,3], r = [2,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">7</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The first drone delivers at hour 3 (recharges at hours 2, 4, 6).</li>
	<li>The second drone delivers at hours 1, 5, 7 (recharges at hours 2, 4, 6).</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">d = [2,1], r = [3,4]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The first drone delivers at hours 1, 2 (recharges at hour 3).</li>
	<li>The second drone delivers at hour 3.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>d = [d<sub>1</sub>, d<sub>2</sub>]</code></li>
	<li><code>1 &lt;= d<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
	<li><code>r = [r<sub>1</sub>, r<sub>2</sub>]</code></li>
	<li><code>2 &lt;= r<sub>i</sub> &lt;= 3 * 10<sup>4</sup></code></li>
</ul>


## Hints

1. Use binary search on the total time <code>T</code>.
2. At hours divisible by <code>lcm(r1, r2)</code>, both drones are recharging (unavailable).
3. For a fixed <code>T</code>, recharge counts are <code>floor(T / r1)</code> and <code>floor(T / r2)</code>.
4. Available hours: <code>c1 = T - floor(T / r1)</code>, <code>c2 = T - floor(T / r2)</code>; shared hours = <code>T - floor(T / r1) - floor(T / r2) + floor(T / lcm(r1,r2))</code>.
5. Assign each drone its exclusive/available hours first; remaining deliveries must fit into the <code>shared</code> hours.

## Solution

```rust
impl Solution { pub fn minimum_time(black_d: Vec<i32>, black_r: Vec<i32>) -> i64 { let (black_d1, black_d2, black_r1, black_r2) = (black_d[0] as i64, black_d[1] as i64, black_r[0] as i64, black_r[1] as i64); let black_lcm = (black_r1 * black_r2) / { fn gcd(a: i64, b: i64) -> i64 { if b == 0 { a } else { gcd(b, a % b) } } gcd(black_r1, black_r2) }; let (mut black_low, mut black_high, mut black_ans) = (1i64, 2_000_000_000_000_000i64, 2_000_000_000_000_000i64); while black_low <= black_high { let black_mid = black_low + (black_high - black_low) / 2; if (black_mid - black_mid / black_r1) >= black_d1 && (black_mid - black_mid / black_r2) >= black_d2 && (black_mid - black_mid / black_lcm) >= (black_d1 + black_d2) { black_ans = black_mid; black_high = black_mid - 1; } else { black_low = black_mid + 1; } } black_ans } }
```