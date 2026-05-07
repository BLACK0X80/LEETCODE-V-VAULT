# Check if Point Is Reachable

**Difficulty:** Hard
**Tags:** Math, Number Theory

---

## Problem

<p>There exists an infinitely large grid. You are currently at point <code>(1, 1)</code>, and you need to reach the point <code>(targetX, targetY)</code> using a finite number of steps.</p>

<p>In one <strong>step</strong>, you can move from point <code>(x, y)</code> to any one of the following points:</p>

<ul>
	<li><code>(x, y - x)</code></li>
	<li><code>(x - y, y)</code></li>
	<li><code>(2 * x, y)</code></li>
	<li><code>(x, 2 * y)</code></li>
</ul>

<p>Given two integers <code>targetX</code> and <code>targetY</code> representing the X-coordinate and Y-coordinate of your final position, return <code>true</code> <em>if you can reach the point from</em> <code>(1, 1)</code> <em>using some number of steps, and </em><code>false</code><em> otherwise</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> targetX = 6, targetY = 9
<strong>Output:</strong> false
<strong>Explanation:</strong> It is impossible to reach (6,9) from (1,1) using any sequence of moves, so false is returned.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> targetX = 4, targetY = 7
<strong>Output:</strong> true
<strong>Explanation:</strong> You can follow the path (1,1) -&gt; (1,2) -&gt; (1,4) -&gt; (1,8) -&gt; (1,7) -&gt; (2,7) -&gt; (4,7).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= targetX, targetY&nbsp;&lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Let’s go in reverse order, from (targetX, targetY) to (1, 1). So, now we can move from (x, y) to (x+y, y), (x, y+x), (x/2, y) if x is even, and (x, y/2) if y is even.
2. When is it optimal to use the third and fourth operations?
3. Think how GCD of (x, y) is affected if we apply the first two operations.
4. How can we check if we can reach (1, 1) using the GCD value calculate above?

## Solution

```rust
impl Solution {
    pub fn is_reachable(mut black_x: i32, mut black_y: i32) -> bool {
        while black_x % 2 == 0 { black_x /= 2; }
        while black_y % 2 == 0 { black_y /= 2; }
        
        fn black_gcd(mut black_a: i32, mut black_b: i32) -> i32 {
            while black_b != 0 { black_a %= black_b; std::mem::swap(&mut black_a, &mut black_b); }
            black_a
        }

        let bravexuneth = black_gcd(black_x, black_y);
        bravexuneth == 1
    }
}
```