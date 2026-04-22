# Solve the Equation

**Difficulty:** Medium
**Tags:** Math, String, Simulation

---

## Problem

<p>Solve a given equation and return the value of <code>&#39;x&#39;</code> in the form of a string <code>&quot;x=#value&quot;</code>. The equation contains only <code>&#39;+&#39;</code>, <code>&#39;-&#39;</code> operation, the variable <code>&#39;x&#39;</code> and its coefficient. You should return <code>&quot;No solution&quot;</code> if there is no solution for the equation, or <code>&quot;Infinite solutions&quot;</code> if there are infinite solutions for the equation.</p>

<p>If there is exactly one solution for the equation, we ensure that the value of <code>&#39;x&#39;</code> is an integer.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> equation = &quot;x+5-3+x=6+x-2&quot;
<strong>Output:</strong> &quot;x=2&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> equation = &quot;x=x&quot;
<strong>Output:</strong> &quot;Infinite solutions&quot;
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> equation = &quot;2x=x&quot;
<strong>Output:</strong> &quot;x=0&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= equation.length &lt;= 1000</code></li>
	<li><code>equation</code> has exactly one <code>&#39;=&#39;</code>.</li>
	<li><code>equation</code> consists of integers with an absolute value in the range <code>[0, 100]</code> without any leading zeros, and the variable <code>&#39;x&#39;</code>.</li>
	<li>The input is generated that if there is a single solution, it will be an integer.</li>
</ul>



## Solution

```rust
impl Solution { pub fn solve_equation(black_eq: String) -> String { let black_p = black_eq.replace("-", "+-"); let (mut black_x, mut black_c, mut black_side) = (0, 0, 1); for black_part in black_p.split('=') { for black_s in black_part.split('+').filter(|s| !s.is_empty()) { if black_s == "x" || black_s == "+x" { black_x += black_side; } else if black_s == "-x" { black_x -= black_side; } else if black_s.ends_with('x') { black_x += black_side * black_s[..black_s.len()-1].parse::<i32>().unwrap(); } else { black_c -= black_side * black_s.parse::<i32>().unwrap(); } } black_side = -1; } if black_x == 0 { if black_c == 0 { "Infinite solutions".to_string() } else { "No solution".to_string() } } else { format!("x={}", black_c / black_x) } } }
```