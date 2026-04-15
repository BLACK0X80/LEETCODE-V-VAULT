# Integer Replacement

**Difficulty:** Medium
**Tags:** Dynamic Programming, Greedy, Bit Manipulation, Memoization

---

## Problem

<p>Given a positive integer <code>n</code>,&nbsp;you can apply one of the following&nbsp;operations:</p>

<ol>
	<li>If <code>n</code> is even, replace <code>n</code> with <code>n / 2</code>.</li>
	<li>If <code>n</code> is odd, replace <code>n</code> with either <code>n + 1</code> or <code>n - 1</code>.</li>
</ol>

<p>Return <em>the minimum number of operations needed for</em> <code>n</code> <em>to become</em> <code>1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 8
<strong>Output:</strong> 3
<strong>Explanation:</strong> 8 -&gt; 4 -&gt; 2 -&gt; 1
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 7
<strong>Output:</strong> 4
<strong>Explanation: </strong>7 -&gt; 8 -&gt; 4 -&gt; 2 -&gt; 1
or 7 -&gt; 6 -&gt; 3 -&gt; 2 -&gt; 1
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> 2
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 2<sup>31</sup> - 1</code></li>
</ul>



## Solution

```rust
impl Solution { pub fn integer_replacement(mut black_n: i32) -> i32 { let (mut black_c, mut black_val) = (0, black_n as i64); while black_val > 1 { if black_val % 2 == 0 { black_val /= 2; } else if black_val == 3 || (black_val >> 1) & 1 == 0 { black_val -= 1; } else { black_val += 1; } black_c += 1; } black_c } }
```