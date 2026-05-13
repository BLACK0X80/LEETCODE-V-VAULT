# Find Maximum Number of Non Intersecting Substrings

**Difficulty:** Medium
**Tags:** Hash Table, String, Dynamic Programming, Greedy

---

## Problem

<p>You are given a string <code>word</code>.</p>

<p>Return the <strong>maximum</strong> number of non-intersecting <strong><span data-keyword="substring-nonempty">substrings</span></strong> of word that are at <strong>least</strong> four characters long and start and end with the same letter.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word = &quot;abcdeafdef&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The two substrings are <code>&quot;abcdea&quot;</code> and <code>&quot;fdef&quot;</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word = &quot;bcdaaaab&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The only substring is <code>&quot;aaaa&quot;</code>. Note that we cannot <strong>also</strong> choose <code>&quot;bcdaaaab&quot;</code> since it intersects with the other substring.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= word.length &lt;= 2 * 10<sup>5</sup></code></li>
	<li><code>word</code> consists only of lowercase English letters.</li>
</ul>


## Hints

1. Can we solve the problem using Dynamic Programming?
2. For each character <code>c</code>, store all occurrence indices in order
3. At each position <code>i</code>, let <code>j</code> be the first index of <code>word[i]</code>; if <code>i - j >= 3</code>, we can form substring <code>[j, i]</code>
4. For each index, also store the maximum for <b>any</b> substring ending before that index in the dp.

## Solution

```rust

```