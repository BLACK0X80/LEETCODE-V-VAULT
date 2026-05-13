# Prime Pairs With Target Sum

**Difficulty:** Medium
**Tags:** Array, Math, Enumeration, Number Theory

---

## Problem

<p>You are given an integer <code>n</code>. We say that two integers <code>x</code> and <code>y</code> form a prime number pair if:</p>

<ul>
	<li><code>1 &lt;= x &lt;= y &lt;= n</code></li>
	<li><code>x + y == n</code></li>
	<li><code>x</code> and <code>y</code> are prime numbers</li>
</ul>

<p>Return <em>the 2D sorted list of prime number pairs</em> <code>[x<sub>i</sub>, y<sub>i</sub>]</code>. The list should be sorted in <strong>increasing</strong> order of <code>x<sub>i</sub></code>. If there are no prime number pairs at all, return <em>an empty array</em>.</p>

<p><strong>Note:</strong> A prime number is a natural number greater than <code>1</code> with only two factors, itself and <code>1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> [[3,7],[5,5]]
<strong>Explanation:</strong> In this example, there are two prime pairs that satisfy the criteria. 
These pairs are [3,7] and [5,5], and we return them in the sorted order as described in the problem statement.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> []
<strong>Explanation:</strong> We can show that there is no prime number pair that gives a sum of 2, so we return an empty array. 
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Pre-compute all the prime numbers in the range [1, n] using a sieve, and store them in a data structure where they can be accessed in O(1) time.
2. For x in the range [2, n/2], we can use the pre-computed list of prime numbers to check if both x and n - x are primes. If they are, we add them to the result.

## Solution

```rust

```