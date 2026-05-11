# Can Make Palindrome from Substring

**Difficulty:** Medium
**Tags:** Array, Hash Table, String, Bit Manipulation, Prefix Sum

---

## Problem

<p>You are given a string <code>s</code> and array <code>queries</code> where <code>queries[i] = [left<sub>i</sub>, right<sub>i</sub>, k<sub>i</sub>]</code>. We may rearrange the substring <code>s[left<sub>i</sub>...right<sub>i</sub>]</code> for each query and then choose up to <code>k<sub>i</sub></code> of them to replace with any lowercase English letter.</p>

<p>If the substring is possible to be a palindrome string after the operations above, the result of the query is <code>true</code>. Otherwise, the result is <code>false</code>.</p>

<p>Return a boolean array <code>answer</code> where <code>answer[i]</code> is the result of the <code>i<sup>th</sup></code> query <code>queries[i]</code>.</p>

<p>Note that each letter is counted individually for replacement, so if, for example <code>s[left<sub>i</sub>...right<sub>i</sub>] = &quot;aaa&quot;</code>, and <code>k<sub>i</sub> = 2</code>, we can only replace two of the letters. Also, note that no query modifies the initial string <code>s</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example :</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcda&quot;, queries = [[3,3,0],[1,2,0],[0,3,1],[0,3,2],[0,4,1]]
<strong>Output:</strong> [true,false,false,true,true]
<strong>Explanation:</strong>
queries[0]: substring = &quot;d&quot;, is palidrome.
queries[1]: substring = &quot;bc&quot;, is not palidrome.
queries[2]: substring = &quot;abcd&quot;, is not palidrome after replacing only 1 character.
queries[3]: substring = &quot;abcd&quot;, could be changed to &quot;abba&quot; which is palidrome. Also this can be changed to &quot;baab&quot; first rearrange it &quot;bacd&quot; then replace &quot;cd&quot; with &quot;ab&quot;.
queries[4]: substring = &quot;abcda&quot;, could be changed to &quot;abcba&quot; which is palidrome.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;lyb&quot;, queries = [[0,1,0],[2,2,1]]
<strong>Output:</strong> [false,true]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length, queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= left<sub>i</sub> &lt;= right<sub>i</sub> &lt; s.length</code></li>
	<li><code>0 &lt;= k<sub>i</sub> &lt;= s.length</code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
</ul>


## Hints

1. Since we can rearrange the substring, all we care about is the frequency of each character in that substring.
2. How to find the character frequencies efficiently ?
3. As a preprocess, calculate the accumulate frequency of all characters for all prefixes of the string.
4. How to check if a substring can be changed to a palindrome given its characters frequency ?
5. Count the number of odd frequencies, there can be at most one odd frequency in a palindrome.

## Solution

```rust
impl Solution { pub fn can_make_pali_queries(black_s: String, black_q: Vec<Vec<i32>>) -> Vec<bool> { let mut black_p = vec![0u32; black_s.len() + 1]; for (black_i, black_c) in black_s.bytes().enumerate() { black_p[black_i+1] = black_p[black_i] ^ (1 << (black_c - b'a')); } black_q.into_iter().map(|black_v| { let black_bits = (black_p[black_v[1] as usize + 1] ^ black_p[black_v[0] as usize]).count_ones(); (black_bits / 2) as i32 <= black_v[2] }).collect() } }
```