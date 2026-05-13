# Generate Binary Strings Without Adjacent Zeros

**Difficulty:** Medium
**Tags:** String, Backtracking, Bit Manipulation

---

## Problem

<p>You are given a positive integer <code>n</code>.</p>

<p>A binary string <code>x</code> is <strong>valid</strong> if all <span data-keyword="substring-nonempty">substrings</span> of <code>x</code> of length 2 contain <strong>at least</strong> one <code>&quot;1&quot;</code>.</p>

<p>Return all <strong>valid</strong> strings with length <code>n</code><strong>, </strong>in <em>any</em> order.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">[&quot;010&quot;,&quot;011&quot;,&quot;101&quot;,&quot;110&quot;,&quot;111&quot;]</span></p>

<p><strong>Explanation:</strong></p>

<p>The valid strings of length 3 are: <code>&quot;010&quot;</code>, <code>&quot;011&quot;</code>, <code>&quot;101&quot;</code>, <code>&quot;110&quot;</code>, and <code>&quot;111&quot;</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">[&quot;0&quot;,&quot;1&quot;]</span></p>

<p><strong>Explanation:</strong></p>

<p>The valid strings of length 1 are: <code>&quot;0&quot;</code> and <code>&quot;1&quot;</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 18</code></li>
</ul>


## Hints

1. If we have a string <code>s</code> of length <code>x</code>, we can generate all strings of length <code>x + 1</code>.
2. If <code>s</code> has 0 as the last character, we can only append 1, whereas if the last character is 1, we can append both 0 and 1.
3. We can use recursion and backtracking to generate all such strings.

## Solution

```rust

```