# Smallest String With A Given Numeric Value

**Difficulty:** Medium
**Tags:** String, Greedy

---

## Problem

<p>The <strong>numeric value</strong> of a <strong>lowercase character</strong> is defined as its position <code>(1-indexed)</code> in the alphabet, so the numeric value of <code>a</code> is <code>1</code>, the numeric value of <code>b</code> is <code>2</code>, the numeric value of <code>c</code> is <code>3</code>, and so on.</p>

<p>The <strong>numeric value</strong> of a <strong>string</strong> consisting of lowercase characters is defined as the sum of its characters&#39; numeric values. For example, the numeric value of the string <code>&quot;abe&quot;</code> is equal to <code>1 + 2 + 5 = 8</code>.</p>

<p>You are given two integers <code>n</code> and <code>k</code>. Return <em>the <strong>lexicographically smallest string</strong> with <strong>length</strong> equal to <code>n</code> and <strong>numeric value</strong> equal to <code>k</code>.</em></p>

<p>Note that a string <code>x</code> is lexicographically smaller than string <code>y</code> if <code>x</code> comes before <code>y</code> in dictionary order, that is, either <code>x</code> is a prefix of <code>y</code>, or if <code>i</code> is the first position such that <code>x[i] != y[i]</code>, then <code>x[i]</code> comes before <code>y[i]</code> in alphabetic order.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 3, k = 27
<strong>Output:</strong> &quot;aay&quot;
<strong>Explanation:</strong> The numeric value of the string is 1 + 1 + 25 = 27, and it is the smallest string with such a value and length equal to 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 5, k = 73
<strong>Output:</strong> &quot;aaszz&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>n &lt;= k &lt;= 26 * n</code></li>
</ul>


## Hints

1. Think greedily.
2. If you build the string from the end to the beginning, it will always be optimal to put the highest possible character at the current index.

## Solution

```rust
impl Solution { pub fn get_smallest_string(black_n: i32, mut black_k: i32) -> String { let mut black_res = vec![b'a'; black_n as usize]; black_k -= black_n; for black_i in (0..black_n as usize).rev() { let black_add = black_k.min(25); black_res[black_i] += black_add as u8; black_k -= black_add; } String::from_utf8(black_res).unwrap() } }
```