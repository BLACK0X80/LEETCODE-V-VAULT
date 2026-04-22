# Remove K Digits

**Difficulty:** Medium
**Tags:** String, Stack, Greedy, Monotonic Stack

---

## Problem

<p>Given string num representing a non-negative integer <code>num</code>, and an integer <code>k</code>, return <em>the smallest possible integer after removing</em> <code>k</code> <em>digits from</em> <code>num</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;1432219&quot;, k = 3
<strong>Output:</strong> &quot;1219&quot;
<strong>Explanation:</strong> Remove the three digits 4, 3, and 2 to form the new number 1219 which is the smallest.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;10200&quot;, k = 1
<strong>Output:</strong> &quot;200&quot;
<strong>Explanation:</strong> Remove the leading 1 and the number is 200. Note that the output must not contain leading zeroes.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;10&quot;, k = 2
<strong>Output:</strong> &quot;0&quot;
<strong>Explanation:</strong> Remove all the digits from the number and it is left with nothing which is 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= num.length &lt;= 10<sup>5</sup></code></li>
	<li><code>num</code> consists of only digits.</li>
	<li><code>num</code> does not have any leading zeros except for the zero itself.</li>
</ul>



## Solution

```rust
impl Solution { pub fn remove_kdigits(black_num: String, mut black_k: i32) -> String { let mut black_stack = vec![]; for c in black_num.chars() { while black_k > 0 && !black_stack.is_empty() && *black_stack.last().unwrap() > c { black_stack.pop(); black_k -= 1; } black_stack.push(c); } for _ in 0..black_k { black_stack.pop(); } let black_res: String = black_stack.into_iter().skip_while(|&c| c == '0').collect(); if black_res.is_empty() { "0".to_string() } else { black_res } } }
```