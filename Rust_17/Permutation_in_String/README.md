# Permutation in String

**Difficulty:** Medium
**Tags:** Hash Table, Two Pointers, String, Sliding Window

---

## Problem

<p>Given two strings <code>s1</code> and <code>s2</code>, return <code>true</code> if <code>s2</code> contains a <span data-keyword="permutation-string">permutation</span> of <code>s1</code>, or <code>false</code> otherwise.</p>

<p>In other words, return <code>true</code> if one of <code>s1</code>&#39;s permutations is the substring of <code>s2</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;ab&quot;, s2 = &quot;eidbaooo&quot;
<strong>Output:</strong> true
<strong>Explanation:</strong> s2 contains one permutation of s1 (&quot;ba&quot;).
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;ab&quot;, s2 = &quot;eidboaoo&quot;
<strong>Output:</strong> false
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s1.length, s2.length &lt;= 10<sup>4</sup></code></li>
	<li><code>s1</code> and <code>s2</code> consist of lowercase English letters.</li>
</ul>


## Hints

1. Obviously, brute force will result in TLE. Think of something else.
2. How will you check whether one string is a permutation of another string?
3. One way is to sort the string and then compare. But, Is there a better way?
4. If one string is a permutation of another string then they must have one common metric. What is that?
5. Both strings must have same character frequencies, if  one is permutation of another. Which data structure should be used to store frequencies?
6. What about hash table?  An array of size 26?

## Solution

```rust
impl Solution { pub fn check_inclusion(black_s1: String, black_s2: String) -> bool { if black_s1.len() > black_s2.len() { return false; } let (mut black_c1, mut black_c2, black_b1, black_b2) = ([0i32; 26], [0i32; 26], black_s1.as_bytes(), black_s2.as_bytes()); for black_i in 0..black_b1.len() { black_c1[(black_b1[black_i] - b'a') as usize] += 1; black_c2[(black_b2[black_i] - b'a') as usize] += 1; } if black_c1 == black_c2 { return true; } for black_i in black_b1.len()..black_b2.len() { black_c2[(black_b2[black_i] - b'a') as usize] += 1; black_c2[(black_b2[black_i - black_b1.len()] - b'a') as usize] -= 1; if black_c1 == black_c2 { return true; } } false } }
```