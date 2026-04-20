# Least Operators to Express Number

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming, Memoization

---

## Problem

<p>Given a single positive integer <code>x</code>, we will write an expression of the form <code>x (op1) x (op2) x (op3) x ...</code> where each operator <code>op1</code>, <code>op2</code>, etc. is either addition, subtraction, multiplication, or division (<code>+</code>, <code>-</code>, <code>*</code>, or <code>/)</code>. For example, with <code>x = 3</code>, we might write <code>3 * 3 / 3 + 3 - 3</code> which is a value of <font face="monospace">3</font>.</p>

<p>When writing such an expression, we adhere to the following conventions:</p>

<ul>
	<li>The division operator (<code>/</code>) returns rational numbers.</li>
	<li>There are no parentheses placed anywhere.</li>
	<li>We use the usual order of operations: multiplication and division happen before addition and subtraction.</li>
	<li>It is not allowed to use the unary negation operator (<code>-</code>). For example, &quot;<code>x - x</code>&quot; is a valid expression as it only uses subtraction, but &quot;<code>-x + x</code>&quot; is not because it uses negation.</li>
</ul>

<p>We would like to write an expression with the least number of operators such that the expression equals the given <code>target</code>. Return the least number of operators used.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> x = 3, target = 19
<strong>Output:</strong> 5
<strong>Explanation:</strong> 3 * 3 + 3 * 3 + 3 / 3.
The expression contains 5 operations.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> x = 5, target = 501
<strong>Output:</strong> 8
<strong>Explanation:</strong> 5 * 5 * 5 * 5 - 5 * 5 * 5 + 5 / 5.
The expression contains 8 operations.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> x = 100, target = 100000000
<strong>Output:</strong> 3
<strong>Explanation:</strong> 100 * 100 * 100 * 100.
The expression contains 3 operations.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= x &lt;= 100</code></li>
	<li><code>1 &lt;= target &lt;= 2 * 10<sup>8</sup></code></li>
</ul>



## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
        let mut memo = HashMap::new();
        (dp(0, target as i64, x as i64, &mut memo) - 1) as i32
    }
}

fn dp(i: i64, t: i64, x: i64, memo: &mut HashMap<(i64,i64), i64>) -> i64 {
    if t == 0 { return 0; }
    if t == 1 { return if i == 0 { 2 } else { i }; }
    if i >= 40 { return i64::MAX / 2; }
    if let Some(&v) = memo.get(&(i, t)) { return v; }
    let cost = if i == 0 { 2 } else { i };
    let (q, r) = (t / x, t % x);
    let res = (r * cost + dp(i+1, q, x, memo))
        .min((x - r) * cost + dp(i+1, q+1, x, memo));
    memo.insert((i, t), res);
    res
}
```