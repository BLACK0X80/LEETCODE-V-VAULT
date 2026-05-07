# Palindrome Pairs

**Difficulty:** Hard
**Tags:** Array, Hash Table, String, Trie, Hash Function

---

## Problem

<p>You are given a <strong>0-indexed</strong> array of <strong>unique</strong> strings <code>words</code>.</p>

<p>A <strong>palindrome pair</strong> is a pair of integers <code>(i, j)</code> such that:</p>

<ul>
	<li><code>0 &lt;= i, j &lt; words.length</code>,</li>
	<li><code>i != j</code>, and</li>
	<li><code>words[i] + words[j]</code> (the concatenation of the two strings) is a <span data-keyword="palindrome-string">palindrome</span>.</li>
</ul>

<p>Return <em>an array of all the <strong>palindrome pairs</strong> of </em><code>words</code>.</p>

<p>You must write an algorithm with&nbsp;<code>O(sum of words[i].length)</code>&nbsp;runtime complexity.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;abcd&quot;,&quot;dcba&quot;,&quot;lls&quot;,&quot;s&quot;,&quot;sssll&quot;]
<strong>Output:</strong> [[0,1],[1,0],[3,2],[2,4]]
<strong>Explanation:</strong> The palindromes are [&quot;abcddcba&quot;,&quot;dcbaabcd&quot;,&quot;slls&quot;,&quot;llssssll&quot;]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;bat&quot;,&quot;tab&quot;,&quot;cat&quot;]
<strong>Output:</strong> [[0,1],[1,0]]
<strong>Explanation:</strong> The palindromes are [&quot;battab&quot;,&quot;tabbat&quot;]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;a&quot;,&quot;&quot;]
<strong>Output:</strong> [[0,1],[1,0]]
<strong>Explanation:</strong> The palindromes are [&quot;a&quot;,&quot;a&quot;]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 5000</code></li>
	<li><code>0 &lt;= words[i].length &lt;= 300</code></li>
	<li><code>words[i]</code> consists of lowercase English letters.</li>
</ul>


## Hints

1. Checking every two pairs will exceed the time limit. It will be O(n^2 * k). We need a faster way.
2. If we hash every string in the array, how can we check if two pairs form a palindrome after the concatenation?
3. We can check every string in words and consider it as words[j] (i.e., the suffix of the target palindrome). We can check if there is a hash of string that can be the prefix to make it a palindrome.

## Solution

```rust

```