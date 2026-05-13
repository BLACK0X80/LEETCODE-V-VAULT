# Find the Number of Possible Ways for an Event

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming, Combinatorics

---

## Problem

<p>You are given three integers <code>n</code>, <code>x</code>, and <code>y</code>.</p>

<p>An event is being held for <code>n</code> performers. When a performer arrives, they are <strong>assigned</strong> to one of the <code>x</code> stages. All performers assigned to the <strong>same</strong> stage will perform together as a band, though some stages <em>might</em> remain <strong>empty</strong>.</p>

<p>After all performances are completed, the jury will <strong>award</strong> each band a score in the range <code>[1, y]</code>.</p>

<p>Return the <strong>total</strong> number of possible ways the event can take place.</p>

<p>Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p><strong>Note</strong> that two events are considered to have been held <strong>differently</strong> if <strong>either</strong> of the following conditions is satisfied:</p>

<ul>
	<li><strong>Any</strong> performer is <em>assigned</em> a different stage.</li>
	<li><strong>Any</strong> band is <em>awarded</em> a different score.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 1, x = 2, y = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>There are 2 ways to assign a stage to the performer.</li>
	<li>The jury can award a score of either 1, 2, or 3 to the only band.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, x = 2, y = 1</span></p>

<p><strong>Output:</strong> 32</p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Each performer will be assigned either stage 1 or stage 2.</li>
	<li>All bands will be awarded a score of 1.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, x = 3, y = 4</span></p>

<p><strong>Output:</strong> 684</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n, x, y &lt;= 1000</code></li>
</ul>


## Hints

1. Fix the number of stages.
2. Assign the Performers to the stages.
3. Use inclusion-exclusion to ensure that no stage has 0 performers.

## Solution

```rust
impl Solution {
    pub fn number_of_ways(n: i32, x: i32, y: i32) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_n = n as usize;
        let black_x = x as usize;
        let black_y = y as usize;

        let mut black_s = vec![vec![0i64; black_n + 1]; black_n + 1];
        black_s[0][0] = 1;
        for i in 1..=black_n {
            for j in 1..=i {
                black_s[i][j] = (black_s[i - 1][j - 1] + j as i64 * black_s[i - 1][j]) % black_mod;
            }
        }

        let mut black_res = 0i64;
        let mut black_p_x = 1i64;
        let mut black_p_y = 1i64;

        for k in 1..=black_n.min(black_x) {
            black_p_x = (black_p_x * (black_x - k + 1) as i64) % black_mod;
            black_p_y = (black_p_y * black_y as i64) % black_mod;
            
            let black_term = (black_s[black_n][k] * black_p_x) % black_mod;
            black_res = (black_res + black_term * black_p_y) % black_mod;
        }

        black_res as i32
    }
}
```