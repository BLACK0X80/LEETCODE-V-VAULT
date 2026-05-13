# Sum of K-Digit Numbers in a Range

**Difficulty:** Hard
**Tags:** Math, Divide and Conquer, Combinatorics, Number Theory

---

## Problem

<p>You are given three integers <code>l</code>, <code>r</code>, and <code>k</code>.</p>

<p>Consider all possible integers consisting of <strong>exactly</strong> <code>k</code> digits, where each digit is chosen independently from the integer range <code>[l, r]</code> (inclusive). If 0 is included in the range, leading zeros are allowed.</p>

<p>Return an integer representing the <b>sum of all such numbers.</b>​​​​​​​ Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = 1, r = 2, k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">66</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>All numbers formed using <code>k = 2</code> digits in the range <code>[1, 2]</code> are <code>11, 12, 21, 22</code>.</li>
	<li>The total sum is <code>11 + 12 + 21 + 22 = 66</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = 0, r = 1, k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">444</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>All numbers formed using <code>k = 3</code> digits in the range <code>[0, 1]</code> are <code>000, 001, 010, 011, 100, 101, 110, 111</code>​​​​​​​.</li>
	<li>These numbers without leading zeros are <code>0, 1, 10, 11, 100, 101, 110, 111</code>.</li>
	<li>The total sum is 444.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = 5, r = 5, k = 10</span></p>

<p><strong>Output:</strong> <span class="example-io">555555520</span></p>

<p><strong>Explanation:</strong>​​​​​​​</p>

<ul>
	<li>5555555555 is the only valid number consisting of <code>k = 10</code> digits in the range <code>[5, 5]</code>.</li>
	<li>The total sum is <code>5555555555 % (10<sup>9</sup> + 7) = 555555520</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= l &lt;= r &lt;= 9</code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Use combinatorics.
2. For some position <code>p</code> among the <code>k</code> digits, the sum is <code>10^p * (l + (l+1) + ... + r) * (r - l + 1)^(k-1)</code>.
3. Summing <code>p</code> over <code>[0, k-1]</code> gives <code>(l + (l+1) + ... + r) * (r - l + 1)^(k-1) * (10^k - 1) / 9</code>.

## Solution

```rust

```