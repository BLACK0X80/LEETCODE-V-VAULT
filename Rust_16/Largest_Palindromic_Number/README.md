# Largest Palindromic Number

**Difficulty:** Medium
**Tags:** Hash Table, String, Greedy, Counting

---

## Problem

<p>You are given a string <code>num</code> consisting of digits only.</p>

<p>Return <em>the <strong>largest palindromic</strong> integer (in the form of a string) that can be formed using digits taken from </em><code>num</code>. It should not contain <strong>leading zeroes</strong>.</p>

<p><strong>Notes:</strong></p>

<ul>
	<li>You do <strong>not</strong> need to use all the digits of <code>num</code>, but you must use <strong>at least</strong> one digit.</li>
	<li>The digits can be reordered.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;444947137&quot;
<strong>Output:</strong> &quot;7449447&quot;
<strong>Explanation:</strong> 
Use the digits &quot;4449477&quot; from &quot;<u><strong>44494</strong></u><u><strong>7</strong></u>13<u><strong>7</strong></u>&quot; to form the palindromic integer &quot;7449447&quot;.
It can be shown that &quot;7449447&quot; is the largest palindromic integer that can be formed.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;00009&quot;
<strong>Output:</strong> &quot;9&quot;
<strong>Explanation:</strong> 
It can be shown that &quot;9&quot; is the largest palindromic integer that can be formed.
Note that the integer returned should not contain leading zeroes.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num.length &lt;= 10<sup>5</sup></code></li>
	<li><code>num</code> consists of digits.</li>
</ul>


## Hints

1. In order to form a valid palindrome, other than the middle digit in an odd-length palindrome, every digit needs to exist on both sides.
2. A longer palindrome implies a larger valued palindrome. For palindromes of the same length, the larger digits should occur first.
3. We can count the occurrences of each digit and build the palindrome starting from the ends. Starting from the larger digits, if there are still at least 2 occurrences of a digit, we can place these digits on each side.
4. Make sure to consider the special case for the center digit (if any) and zeroes. There should not be leading zeroes.

## Solution

```rust

```