# Last Substring in Lexicographical Order

**Difficulty:** Hard
**Tags:** Two Pointers, String

---

## Problem

<p>Given a string <code>s</code>, return <em>the last substring of</em> <code>s</code> <em>in lexicographical order</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abab&quot;
<strong>Output:</strong> &quot;bab&quot;
<strong>Explanation:</strong> The substrings are [&quot;a&quot;, &quot;ab&quot;, &quot;aba&quot;, &quot;abab&quot;, &quot;b&quot;, &quot;ba&quot;, &quot;bab&quot;]. The lexicographically maximum substring is &quot;bab&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;leetcode&quot;
<strong>Output:</strong> &quot;tcode&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 4 * 10<sup>5</sup></code></li>
	<li><code>s</code> contains only lowercase English letters.</li>
</ul>


## Hints

1. Assume that the answer is a sub-string from index i to j. If you add the character at index j+1 you get a better answer.
2. The answer is always a suffix of the given string.
3. Since the limits are high, we need an efficient data structure.
4. Use suffix array.

## Solution

```rust

```