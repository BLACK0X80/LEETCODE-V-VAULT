# Decode XORed Permutation

**Difficulty:** Medium
**Tags:** Array, Bit Manipulation

---

## Problem

<p>There is an integer array <code>perm</code> that is a permutation of the first <code>n</code> positive integers, where <code>n</code> is always <strong>odd</strong>.</p>

<p>It was encoded into another integer array <code>encoded</code> of length <code>n - 1</code>, such that <code>encoded[i] = perm[i] XOR perm[i + 1]</code>. For example, if <code>perm = [1,3,2]</code>, then <code>encoded = [2,1]</code>.</p>

<p>Given the <code>encoded</code> array, return <em>the original array</em> <code>perm</code>. It is guaranteed that the answer exists and is unique.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> encoded = [3,1]
<strong>Output:</strong> [1,2,3]
<strong>Explanation:</strong> If perm = [1,2,3], then encoded = [1 XOR 2,2 XOR 3] = [3,1]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> encoded = [6,5,4,6]
<strong>Output:</strong> [2,4,1,5,3]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= n &lt;&nbsp;10<sup>5</sup></code></li>
	<li><code>n</code>&nbsp;is odd.</li>
	<li><code>encoded.length == n - 1</code></li>
</ul>


## Hints

1. Compute the XOR of the numbers between 1 and n, and think about how it can be used. Let it be x.
2. Think why n is odd.
3. perm[0] = x XOR encoded[1] XOR encoded[3] XOR encoded[5] ...
4. perm[i] = perm[i-1] XOR encoded[i-1]

## Solution

```rust
impl Solution { pub fn decode(black_enc: Vec<i32>) -> Vec<i32> { let black_n = black_enc.len() + 1; let mut black_total = 0; for black_i in 1..=black_n { black_total ^= black_i as i32; } let mut black_odd_enc = 0; for black_i in (1..black_enc.len()).step_by(2) { black_odd_enc ^= black_enc[black_i]; } let mut black_res = vec![0; black_n]; black_res[0] = black_total ^ black_odd_enc; for black_i in 0..black_enc.len() { black_res[black_i+1] = black_res[black_i] ^ black_enc[black_i]; } black_res } }
```