# Count Fancy Numbers in a Range

**Difficulty:** Hard
**Tags:** Math, Dynamic Programming

---

## Problem

<p>You are given two integers <code>l</code> and <code>r</code>.</p>

<p>An integer is called <strong>good</strong> if its digits form a <strong>strictly monotone</strong> sequence, meaning the digits are <strong>strictly increasing</strong> or <strong>strictly decreasing</strong>. All single-digit integers are considered good.</p>

<p>An integer is called <strong>fancy</strong> if it is good, or if the <strong>sum of its digits</strong> is good.</p>

<p>Return an integer representing the number of fancy integers in the range <code>[l, r]</code> (inclusive).</p>

<p>A sequence is said to be <strong>strictly increasing</strong> if each element is <strong>strictly greater</strong> than its previous one (if exists).</p>

<p>A sequence is said to be <strong>strictly decreasing</strong> if each element is <strong>strictly less</strong> than its previous one (if exists).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = 8, r = 10</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>8 and 9 are single-digit integers, so they are good and therefore fancy.</li>
	<li>10 has digits <code>[1, 0]</code>, which form a strictly decreasing sequence, so 10 is good and thus fancy.</li>
</ul>

<p>Therefore, the answer is 3.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = 12340, r = 12341</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>12340
	<ul>
		<li>12340 is not good because <code>[1, 2, 3, 4, 0]</code> is not strictly monotone.</li>
		<li>The digit sum is <code>1 + 2 + 3 + 4 + 0 = 10</code>.</li>
		<li>10 is good as it has digits <code>[1, 0]</code>, which is strictly decreasing. Therefore, 12340 is fancy.</li>
	</ul>
	</li>
	<li>12341
	<ul>
		<li>12341 is not good because <code>[1, 2, 3, 4, 1]</code> is not strictly monotone.</li>
		<li>The digit sum is <code>1 + 2 + 3 + 4 + 1 = 11</code>.</li>
		<li>11 is not good as it has digits <code>[1, 1]</code>, which is not strictly monotone. Therefore, 12341 is not fancy.</li>
	</ul>
	</li>
</ul>

<p>Therefore, the answer is 1.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = 123456788, r = 123456788</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>123456788 is not good because its digits are not strictly monotone.</li>
	<li>The digit sum is <code>1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 8 = 44</code>.</li>
	<li>44 is not good as it has digits <code>[4, 4]</code>, which is not strictly monotone. Therefore, 123456788 is not fancy.</li>
</ul>

<p>Therefore, the answer is 0.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= l &lt;= r &lt;= 10<sup>15</sup></code></li>
</ul>


## Hints

1. Precompute <code>goods</code>: all strictly increasing numbers made from digits <code>1-9</code> and all strictly decreasing numbers made from digits <code>0-9</code>, remove duplicates.
2. Precompute <code>good_sums</code>: all integers in <code>[1, 144]</code> whose decimal digits are strictly increasing or strictly decreasing (single-digit counts).
3. Implement digit DP <code>cnt(bound)</code> that counts numbers <= <code>bound</code> whose digit sum is in <code>good_sums</code>.
4. Answer = <code>cnt(r) - cnt(l - 1) + count(goods in [l, r]) - count(g in goods where l <= g <= r and sum(g) in good_sums)</code>.

## Solution

```rust

```