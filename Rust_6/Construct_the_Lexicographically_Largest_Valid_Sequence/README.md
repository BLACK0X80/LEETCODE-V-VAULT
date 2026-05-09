# Construct the Lexicographically Largest Valid Sequence

**Difficulty:** Medium
**Tags:** Array, Backtracking

---

## Problem

<p>Given an integer <code>n</code>, find a sequence with elements in the range <code>[1, n]</code> that satisfies all of the following:</p>

<ul>
	<li>The integer <code>1</code> occurs once in the sequence.</li>
	<li>Each integer between <code>2</code> and <code>n</code> occurs twice in the sequence.</li>
	<li>For every integer <code>i</code> between <code>2</code> and <code>n</code>, the <strong>distance</strong> between the two occurrences of <code>i</code> is exactly <code>i</code>.</li>
</ul>

<p>The <strong>distance</strong> between two numbers on the sequence, <code>a[i]</code> and <code>a[j]</code>, is the absolute difference of their indices, <code>|j - i|</code>.</p>

<p>Return <em>the <strong>lexicographically largest</strong> sequence</em><em>. It is guaranteed that under the given constraints, there is always a solution. </em></p>

<p>A sequence <code>a</code> is lexicographically larger than a sequence <code>b</code> (of the same length) if in the first position where <code>a</code> and <code>b</code> differ, sequence <code>a</code> has a number greater than the corresponding number in <code>b</code>. For example, <code>[0,1,9,0]</code> is lexicographically larger than <code>[0,1,5,6]</code> because the first position they differ is at the third number, and <code>9</code> is greater than <code>5</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> [3,1,2,3,2]
<strong>Explanation:</strong> [2,3,2,1,3] is also a valid sequence, but [3,1,2,3,2] is the lexicographically largest valid sequence.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 5
<strong>Output:</strong> [5,3,1,4,3,5,2,4,2]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 20</code></li>
</ul>


## Hints

1. Heuristic algorithm may work.

## Solution

```rust
impl Solution { pub fn construct_distanced_sequence(black_n: i32) -> Vec<i32> { let mut black_res = vec![0; (2 * black_n - 1) as usize]; let mut black_used = vec![false; (black_n + 1) as usize]; Self::black_solve(0, black_n, &mut black_res, &mut black_used); black_res } fn black_solve(black_i: usize, black_n: i32, black_res: &mut Vec<i32>, black_u: &mut Vec<bool>) -> bool { if black_i == black_res.len() { return true; } if black_res[black_i] != 0 { return Self::black_solve(black_i + 1, black_n, black_res, black_u); } for black_v in (1..=black_n).rev() { let black_j = if black_v > 1 { black_i + black_v as usize } else { black_i }; if !black_u[black_v as usize] && (black_v == 1 || (black_j < black_res.len() && black_res[black_j] == 0)) { black_res[black_i] = black_v; black_res[black_j] = black_v; black_u[black_v as usize] = true; if Self::black_solve(black_i + 1, black_n, black_res, black_u) { return true; } black_u[black_v as usize] = false; black_res[black_i] = 0; black_res[black_j] = 0; } } false } }
```