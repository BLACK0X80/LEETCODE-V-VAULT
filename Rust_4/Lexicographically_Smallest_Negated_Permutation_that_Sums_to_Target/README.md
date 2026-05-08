# Lexicographically Smallest Negated Permutation that Sums to Target

**Difficulty:** Medium
**Tags:** Array, Math, Two Pointers, Greedy, Sorting

---

## Problem

<p>You are given a positive integer <code>n</code> and an integer <code>target</code>.</p>

<p>Return the <strong><span data-keyword="lexicographically-smaller-array">lexicographically smallest</span></strong> array of integers of size <code>n</code> such that:</p>

<ul>
	<li>The <strong>sum</strong> of its elements equals <code>target</code>.</li>
	<li>The <strong>absolute values</strong> of its elements form a <strong>permutation</strong> of size <code>n</code>.</li>
</ul>

<p>If no such array exists, return an empty array.</p>

<p>A <strong>permutation</strong> of size <code>n</code> is a rearrangement of integers <code>1, 2, ..., n</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, target = 0</span></p>

<p><strong>Output:</strong> <span class="example-io">[-3,1,2]</span></p>

<p><strong>Explanation:</strong></p>

<p>The arrays that sum to 0 and whose absolute values form a permutation of size 3 are:</p>

<ul>
	<li><code>[-3, 1, 2]</code></li>
	<li><code>[-3, 2, 1]</code></li>
	<li><code>[-2, -1, 3]</code></li>
	<li><code>[-2, 3, -1]</code></li>
	<li><code>[-1, -2, 3]</code></li>
	<li><code>[-1, 3, -2]</code></li>
	<li><code>[1, -3, 2]</code></li>
	<li><code>[1, 2, -3]</code></li>
	<li><code>[2, -3, 1]</code></li>
	<li><code>[2, 1, -3]</code></li>
	<li><code>[3, -2, -1]</code></li>
	<li><code>[3, -1, -2]</code></li>
</ul>

<p>The lexicographically smallest one is <code>[-3, 1, 2]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 1, target = 10000000000</span></p>

<p><strong>Output:</strong> <span class="example-io">[]</span></p>

<p><strong>Explanation:</strong></p>

<p>There are no arrays that sum to <span class="example-io">10000000000 and whose absolute values form a permutation of size 1. Therefore, the answer is <code>[]</code>.</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>10</sup> &lt;= target &lt;= 10<sup>10</sup></code></li>
</ul>


## Hints

1. Start with all numbers positive: <code>[1, 2, ..., n]</code>. Let <code>S = n * (n + 1) / 2</code>.
2. If <code>target < -S</code> or <code>target > S</code> or <code>(S - target) % 2 == 1</code> then no solution - return <code>[]</code>.
3. Let <code>D = S - target</code> (nonnegative). Flipping <code>x</code> to <code>-x</code> reduces the sum by <code>2*x</code>.
4. For <code>x = n</code> down to <code>1</code>: if <code>2x <= D</code> then flip <code>x</code> and set <code>D -= 2x</code>.
5. Build the array using the chosen signs and sort it in ascending order to obtain the lexicographically smallest result.

## Solution

```rust
impl Solution { pub fn lex_smallest_negated_perm(n: i32, target: i64) -> Vec<i32> { let black_ms = n as i64 * (n as i64 + 1) / 2; if (black_ms + target) % 2 != 0 || black_ms < target || black_ms < -target { return vec![]; } let mut black_p = (black_ms + target) / 2; let mut black_ans = Vec::new(); for black_i in (1..=n).rev() { let black_rem = black_i as i64 * (black_i as i64 - 1) / 2; if black_p > black_rem { black_p -= black_i as i64; black_ans.push(black_i); } else { black_ans.push(-black_i); } } black_ans.sort_unstable(); black_ans } }
```