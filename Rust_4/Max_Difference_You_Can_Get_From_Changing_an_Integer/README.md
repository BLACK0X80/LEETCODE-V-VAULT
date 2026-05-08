# Max Difference You Can Get From Changing an Integer

**Difficulty:** Medium
**Tags:** Math, Greedy

---

## Problem

<p>You are given an integer <code>num</code>. You will apply the following steps to <code>num</code> <strong>two</strong> separate times:</p>

<ul>
	<li>Pick a digit <code>x (0 &lt;= x &lt;= 9)</code>.</li>
	<li>Pick another digit <code>y (0 &lt;= y &lt;= 9)</code>. Note <code>y</code> can be equal to <code>x</code>.</li>
	<li>Replace all the occurrences of <code>x</code> in the decimal representation of <code>num</code> by <code>y</code>.</li>
</ul>

<p>Let <code>a</code> and <code>b</code> be the two results from applying the operation to <code>num</code> <em>independently</em>.</p>

<p>Return <em>the max difference</em> between <code>a</code> and <code>b</code>.</p>

<p>Note that neither <code>a</code> nor <code>b</code> may have any leading zeros, and <strong>must not</strong> be 0.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num = 555
<strong>Output:</strong> 888
<strong>Explanation:</strong> The first time pick x = 5 and y = 9 and store the new integer in a.
The second time pick x = 5 and y = 1 and store the new integer in b.
We have now a = 999 and b = 111 and max difference = 888
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num = 9
<strong>Output:</strong> 8
<strong>Explanation:</strong> The first time pick x = 9 and y = 9 and store the new integer in a.
The second time pick x = 9 and y = 1 and store the new integer in b.
We have now a = 9 and b = 1 and max difference = 8
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num &lt;= 10<sup>8</sup></code></li>
</ul>


## Hints

1. We need to get the max and min value after changing num and the answer is max - min.
2. Use brute force, try all possible changes and keep the minimum and maximum values.

## Solution

```rust
impl Solution { pub fn max_diff(black_num: i32) -> i32 { let black_s = black_num.to_string(); let (mut black_a, mut black_b) = (black_s.clone(), black_s.clone()); if let Some(black_x) = black_s.chars().find(|&c| c != '9') { black_a = black_s.replace(black_x, "9"); } if black_s.starts_with('1') { if let Some(black_x) = black_s.chars().skip(1).find(|&c| c != '0' && c != '1') { black_b = black_s.replace(black_x, "0"); } } else { black_b = black_s.replace(black_s.chars().next().unwrap(), "1"); } black_a.parse::<i32>().unwrap() - black_b.parse::<i32>().unwrap() } }
```