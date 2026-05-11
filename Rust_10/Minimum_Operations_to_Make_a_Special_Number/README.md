# Minimum Operations to Make a Special Number

**Difficulty:** Medium
**Tags:** Math, String, Greedy, Enumeration

---

## Problem

<p>You are given a <strong>0-indexed</strong> string <code>num</code> representing a non-negative integer.</p>

<p>In one operation, you can pick any digit of <code>num</code> and delete it. Note that if you delete all the digits of <code>num</code>, <code>num</code> becomes <code>0</code>.</p>

<p>Return <em>the <strong>minimum number of operations</strong> required to make</em> <code>num</code> <i>special</i>.</p>

<p>An integer <code>x</code> is considered <strong>special</strong> if it is divisible by <code>25</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;2245047&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> Delete digits num[5] and num[6]. The resulting number is &quot;22450&quot; which is special since it is divisible by 25.
It can be shown that 2 is the minimum number of operations required to get a special number.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;2908305&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> Delete digits num[3], num[4], and num[6]. The resulting number is &quot;2900&quot; which is special since it is divisible by 25.
It can be shown that 3 is the minimum number of operations required to get a special number.</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;10&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> Delete digit num[0]. The resulting number is &quot;0&quot; which is special since it is divisible by 25.
It can be shown that 1 is the minimum number of operations required to get a special number.

</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num.length &lt;= 100</code></li>
	<li><code>num</code> only consists of digits <code>&#39;0&#39;</code> through <code>&#39;9&#39;</code>.</li>
	<li><code>num</code> does not contain any leading zeros.</li>
</ul>


## Hints

1. If <code>num</code> contains a single zero digit then the answer is at most <code>n - 1</code>.
2. A number is divisible by <code>25</code> if its last two digits are <code>75</code>, <code>50</code>, <code>25</code>, or <code>00</code>.
3. Iterate over all possible pairs of indices <code>i &lt; j</code> such that <code>num[i] * 10 + num[j]</code> is in <code>[00,25,50,75]</code>. Then, set the answer to <code> min(answer, n - i - 2) </code>.

## Solution

```rust
impl Solution { pub fn minimum_operations(num: String) -> i32 { let (black_s, mut black_res) = (num.as_bytes(), num.len() as i32); if num.contains('0') { black_res -= 1; } for black_t in ["00", "25", "50", "75"] { let black_b = black_t.as_bytes(); if let Some(j) = (0..black_s.len()).rev().find(|&k| black_s[k] == black_b[1]) { if let Some(k) = (0..j).rev().find(|&l| black_s[l] == black_b[0]) { black_res = black_res.min((black_s.len() - k - 2) as i32); } } } black_res } }
```