# Longest Palindrome by Concatenating Two Letter Words

**Difficulty:** Medium
**Tags:** Array, Hash Table, String, Greedy, Counting

---

## Problem

<p>You are given an array of strings <code>words</code>. Each element of <code>words</code> consists of <strong>two</strong> lowercase English letters.</p>

<p>Create the <strong>longest possible palindrome</strong> by selecting some elements from <code>words</code> and concatenating them in <strong>any order</strong>. Each element can be selected <strong>at most once</strong>.</p>

<p>Return <em>the <strong>length</strong> of the longest palindrome that you can create</em>. If it is impossible to create any palindrome, return <code>0</code>.</p>

<p>A <strong>palindrome</strong> is a string that reads the same forward and backward.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;lc&quot;,&quot;cl&quot;,&quot;gg&quot;]
<strong>Output:</strong> 6
<strong>Explanation:</strong> One longest palindrome is &quot;lc&quot; + &quot;gg&quot; + &quot;cl&quot; = &quot;lcggcl&quot;, of length 6.
Note that &quot;clgglc&quot; is another longest palindrome that can be created.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;ab&quot;,&quot;ty&quot;,&quot;yt&quot;,&quot;lc&quot;,&quot;cl&quot;,&quot;ab&quot;]
<strong>Output:</strong> 8
<strong>Explanation:</strong> One longest palindrome is &quot;ty&quot; + &quot;lc&quot; + &quot;cl&quot; + &quot;yt&quot; = &quot;tylcclyt&quot;, of length 8.
Note that &quot;lcyttycl&quot; is another longest palindrome that can be created.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;cc&quot;,&quot;ll&quot;,&quot;xx&quot;]
<strong>Output:</strong> 2
<strong>Explanation:</strong> One longest palindrome is &quot;cc&quot;, of length 2.
Note that &quot;ll&quot; is another longest palindrome that can be created, and so is &quot;xx&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 10<sup>5</sup></code></li>
	<li><code>words[i].length == 2</code></li>
	<li><code>words[i]</code> consists of lowercase English letters.</li>
</ul>


## Hints

1. A palindrome must be mirrored over the center. Suppose we have a palindrome. If we prepend the word "ab" on the left, what must we append on the right to keep it a palindrome?
2. We must append "ba" on the right. The number of times we can do this is the minimum of (occurrences of "ab") and (occurrences of "ba").
3. For words that are already palindromes, e.g. "aa", we can prepend and append these in pairs as described in the previous hint. We can also use exactly one in the middle to form an even longer palindrome.

## Solution

```rust
impl Solution { pub fn longest_palindrome(words: Vec<String>) -> i32 { let (mut black_cnt, mut black_res, mut black_mid) = ([[0; 26]; 26], 0, false); for black_w in words { let black_b = black_w.as_bytes(); let (black_u, black_v) = ((black_b[0] - b'a') as usize, (black_b[1] - b'a') as usize); if black_cnt[black_v][black_u] > 0 { black_res += 4; black_cnt[black_v][black_u] -= 1; } else { black_cnt[black_u][black_v] += 1; } } for black_i in 0..26 { if black_cnt[black_i][black_i] > 0 { black_mid = true; break; } } black_res + if black_mid { 2 } else { 0 } } }
```