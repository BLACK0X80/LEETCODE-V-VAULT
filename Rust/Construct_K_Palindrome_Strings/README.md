# Construct K Palindrome Strings

**Difficulty:** Medium
**Tags:** Hash Table, String, Greedy, Counting

---

## Problem

<p>Given a string <code>s</code> and an integer <code>k</code>, return <code>true</code> if you can use all the characters in <code>s</code> to construct <strong>non-empty</strong> <code>k</code> <span data-keyword="palindrome-string">palindrome strings</span> or <code>false</code> otherwise.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;annabelle&quot;, k = 2
<strong>Output:</strong> true
<strong>Explanation:</strong> You can construct two palindromes using all characters in s.
Some possible constructions &quot;anna&quot; + &quot;elble&quot;, &quot;anbna&quot; + &quot;elle&quot;, &quot;anellena&quot; + &quot;b&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;leetcode&quot;, k = 3
<strong>Output:</strong> false
<strong>Explanation:</strong> It is impossible to construct 3 palindromes using all the characters of s.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;true&quot;, k = 4
<strong>Output:</strong> true
<strong>Explanation:</strong> The only possible solution is to put each character in a separate string.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
	<li><code>1 &lt;= k &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. If the s.length < k we cannot construct k strings from s and answer is false.
2. If the number of characters that have odd counts is > k then the minimum number of palindrome strings we can construct is > k and answer is false.
3. Otherwise you can construct exactly k palindrome strings and answer is true (why ?).

## Solution

```rust
impl Solution { pub fn can_construct(black_s: String, black_k: i32) -> bool { if black_s.len() < black_k as usize { return false; } let mut black_counts = [0; 26]; black_s.bytes().for_each(|black_b| black_counts[(black_b - b'a') as usize] += 1); let black_odd = black_counts.iter().filter(|&&black_c| black_c % 2 != 0).count(); black_odd <= black_k as usize } }
```