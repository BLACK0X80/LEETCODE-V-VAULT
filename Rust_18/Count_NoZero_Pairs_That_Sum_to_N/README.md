# Count No-Zero Pairs That Sum to N

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming

---

## Problem

<p>A <strong>no-zero</strong> integer is a <strong>positive</strong> integer that <strong>does not contain the digit</strong> 0 in its decimal representation.</p>

<p>Given an integer <code>n</code>, count the number of pairs <code>(a, b)</code> where:</p>

<ul>
	<li><code>a</code> and <code>b</code> are <strong>no-zero</strong> integers.</li>
	<li><code>a + b = n</code></li>
</ul>

<p>Return an integer denoting the number of such pairs.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The only pair is <code>(1, 1)</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The pairs are <code>(1, 2)</code> and <code>(2, 1)</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 11</span></p>

<p><strong>Output:</strong> <span class="example-io">8</span></p>

<p><strong>Explanation:</strong></p>

<p>The pairs are <code>(2, 9)</code>, <code>(3, 8)</code>, <code>(4, 7)</code>, <code>(5, 6)</code>, <code>(6, 5)</code>, <code>(7, 4)</code>, <code>(8, 3)</code>, and <code>(9, 2)</code>. Note that <code>(1, 10)</code> and <code>(10, 1)</code> do not satisfy the conditions because 10 contains 0 in its decimal representation.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>15</sup></code></li>
</ul>


## Hints

1. Use digit DP over the decimal representation of <code>n</code>.
2. At each digit, track whether a carry is present and whether <code>a</code> or <code>b</code> has used a zero so far.
3. Transition by choosing digits for <code>a</code> and <code>b</code> that add up (with carry) to the current digit of <code>n</code>.
4. Subtract the cases where either number ends up being zero-containing when <code>n</code> itself is no-zero.

## Solution

```rust

```