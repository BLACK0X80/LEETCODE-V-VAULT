# Find the Largest Palindrome Divisible by K

**Difficulty:** Hard
**Tags:** Math, String, Dynamic Programming, Greedy, Number Theory

---

## Problem

<p>You are given two <strong>positive</strong> integers <code>n</code> and <code>k</code>.</p>

<p>An integer <code>x</code> is called <strong>k-palindromic</strong> if:</p>

<ul>
	<li><code>x</code> is a <span data-keyword="palindrome-integer">palindrome</span>.</li>
	<li><code>x</code> is divisible by <code>k</code>.</li>
</ul>

<p>Return the<strong> largest</strong> integer having <code>n</code> digits (as a string) that is <strong>k-palindromic</strong>.</p>

<p><strong>Note</strong> that the integer must <strong>not</strong> have leading zeros.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, k = 5</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;595&quot;</span></p>

<p><strong>Explanation:</strong></p>

<p>595 is the largest k-palindromic integer with 3 digits.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 1, k = 4</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;8&quot;</span></p>

<p><strong>Explanation:</strong></p>

<p>4 and 8 are the only k-palindromic integers with 1 digit.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, k = 6</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;89898&quot;</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= 9</code></li>
</ul>


## Hints

1. It must have a solution since we can have all digits equal to <code>k</code>.
2. Use string dp, store modulus along with length of number currently formed.
3. Is it possible to solve greedily using divisibility rules?

## Solution

```rust

```