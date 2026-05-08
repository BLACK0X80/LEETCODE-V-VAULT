# Different Ways to Add Parentheses

**Difficulty:** Medium
**Tags:** Math, String, Dynamic Programming, Recursion, Memoization

---

## Problem

<p>Given a string <code>expression</code> of numbers and operators, return <em>all possible results from computing all the different possible ways to group numbers and operators</em>. You may return the answer in <strong>any order</strong>.</p>

<p>The test cases are generated such that the output values fit in a 32-bit integer and the number of different results does not exceed <code>10<sup>4</sup></code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> expression = &quot;2-1-1&quot;
<strong>Output:</strong> [0,2]
<strong>Explanation:</strong>
((2-1)-1) = 0 
(2-(1-1)) = 2
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> expression = &quot;2*3-4*5&quot;
<strong>Output:</strong> [-34,-14,-10,-10,10]
<strong>Explanation:</strong>
(2*(3-(4*5))) = -34 
((2*3)-(4*5)) = -14 
((2*(3-4))*5) = -10 
(2*((3-4)*5)) = -10 
(((2*3)-4)*5) = 10
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= expression.length &lt;= 20</code></li>
	<li><code>expression</code> consists of digits and the operator <code>&#39;+&#39;</code>, <code>&#39;-&#39;</code>, and <code>&#39;*&#39;</code>.</li>
	<li>All the integer values in the input expression are in the range <code>[0, 99]</code>.</li>
	<li>The integer values in the input expression do not have a leading <code>&#39;-&#39;</code> or <code>&#39;+&#39;</code> denoting the sign.</li>
</ul>



## Solution

```rust
use std::collections::HashMap;
impl Solution { pub fn diff_ways_to_compute(black_e: String) -> Vec<i32> { Self::black_solve(&black_e, &mut HashMap::new()) } fn black_solve(black_s: &str, black_memo: &mut HashMap<String, Vec<i32>>) -> Vec<i32> { if let Some(res) = black_memo.get(black_s) { return res.clone(); } let mut black_res = vec![]; for (i, c) in black_s.chars().enumerate() { if "+-*".contains(c) { let black_l = Self::black_solve(&black_s[..i], black_memo); let black_r = Self::black_solve(&black_s[i+1..], black_memo); for &l in &black_l { for &r in &black_r { black_res.push(match c { '+' => l + r, '-' => l - r, _ => l * r }); } } } } if black_res.is_empty() { black_res.push(black_s.parse().unwrap()); } black_memo.insert(black_s.to_string(), black_res.clone()); black_res } }
```