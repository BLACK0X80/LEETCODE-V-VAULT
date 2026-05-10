# Count Substrings That Differ by One Character

**Difficulty:** Medium
**Tags:** Hash Table, String, Dynamic Programming, Enumeration

---

## Problem

<p>Given two strings <code>s</code> and <code>t</code>, find the number of ways you can choose a non-empty substring of <code>s</code> and replace a <strong>single character</strong> by a different character such that the resulting substring is a substring of <code>t</code>. In other words, find the number of substrings in <code>s</code> that differ from some substring in <code>t</code> by <strong>exactly</strong> one character.</p>

<p>For example, the underlined substrings in <code>&quot;<u>compute</u>r&quot;</code> and <code>&quot;<u>computa</u>tion&quot;</code> only differ by the <code>&#39;e&#39;</code>/<code>&#39;a&#39;</code>, so this is a valid way.</p>

<p>Return <em>the number of substrings that satisfy the condition above.</em></p>

<p>A <strong>substring</strong> is a contiguous sequence of characters within a string.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aba&quot;, t = &quot;baba&quot;
<strong>Output:</strong> 6
<strong>Explanation:</strong> The following are the pairs of substrings from s and t that differ by exactly 1 character:
(&quot;<u>a</u>ba&quot;, &quot;<u>b</u>aba&quot;)
(&quot;<u>a</u>ba&quot;, &quot;ba<u>b</u>a&quot;)
(&quot;ab<u>a</u>&quot;, &quot;<u>b</u>aba&quot;)
(&quot;ab<u>a</u>&quot;, &quot;ba<u>b</u>a&quot;)
(&quot;a<u>b</u>a&quot;, &quot;b<u>a</u>ba&quot;)
(&quot;a<u>b</u>a&quot;, &quot;bab<u>a</u>&quot;)
The underlined portions are the substrings that are chosen from s and t.
</pre>
​​<strong class="example">Example 2:</strong>

<pre>
<strong>Input:</strong> s = &quot;ab&quot;, t = &quot;bb&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> The following are the pairs of substrings from s and t that differ by 1 character:
(&quot;<u>a</u>b&quot;, &quot;<u>b</u>b&quot;)
(&quot;<u>a</u>b&quot;, &quot;b<u>b</u>&quot;)
(&quot;<u>ab</u>&quot;, &quot;<u>bb</u>&quot;)
​​​​The underlined portions are the substrings that are chosen from s and t.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length, t.length &lt;= 100</code></li>
	<li><code>s</code> and <code>t</code> consist of lowercase English letters only.</li>
</ul>


## Hints

1. Take every substring of s, change a character, and see how many substrings of t match that substring.
2. Use a Trie to store all substrings of t as a dictionary.

## Solution

```rust
impl Solution { pub fn count_substrings(s: String, t: String) -> i32 { let (black_s, black_t) = (s.as_bytes(), t.as_bytes()); let (n, m, mut black_ans) = (black_s.len(), black_t.len(), 0); for i in 0..n { for j in 0..m { if black_s[i] != black_t[j] { let (mut black_pre, mut black_suf) = (0, 0); while i > black_pre && j > black_pre && black_s[i-1-black_pre] == black_t[j-1-black_pre] { black_pre += 1; } while i+1+black_suf < n && j+1+black_suf < m && black_s[i+1+black_suf] == black_t[j+1+black_suf] { black_suf += 1; } black_ans += (black_pre as i32 + 1) * (black_suf as i32 + 1); } } } black_ans } }
```