# Count Vowel Strings in Ranges

**Difficulty:** Medium
**Tags:** Array, String, Prefix Sum

---

## Problem

<p>You are given a <strong>0-indexed</strong> array of strings <code>words</code> and a 2D array of integers <code>queries</code>.</p>

<p>Each query <code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>]</code> asks us to find the number of strings present at the indices ranging from <code>l<sub>i</sub></code> to <code>r<sub>i</sub></code> (both <strong>inclusive</strong>) of <code>words</code> that start and end with a vowel.</p>

<p>Return <em>an array </em><code>ans</code><em> of size </em><code>queries.length</code><em>, where </em><code>ans[i]</code><em> is the answer to the </em><code>i</code><sup>th</sup><em> query</em>.</p>

<p><strong>Note</strong> that the vowel letters are <code>&#39;a&#39;</code>, <code>&#39;e&#39;</code>, <code>&#39;i&#39;</code>, <code>&#39;o&#39;</code>, and <code>&#39;u&#39;</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;aba&quot;,&quot;bcb&quot;,&quot;ece&quot;,&quot;aa&quot;,&quot;e&quot;], queries = [[0,2],[1,4],[1,1]]
<strong>Output:</strong> [2,3,0]
<strong>Explanation:</strong> The strings starting and ending with a vowel are &quot;aba&quot;, &quot;ece&quot;, &quot;aa&quot; and &quot;e&quot;.
The answer to the query [0,2] is 2 (strings &quot;aba&quot; and &quot;ece&quot;).
to query [1,4] is 3 (strings &quot;ece&quot;, &quot;aa&quot;, &quot;e&quot;).
to query [1,1] is 0.
We return [2,3,0].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;a&quot;,&quot;e&quot;,&quot;i&quot;], queries = [[0,2],[0,1],[2,2]]
<strong>Output:</strong> [3,2,1]
<strong>Explanation:</strong> Every string satisfies the conditions, so we return [3,2,1].</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= words[i].length &lt;= 40</code></li>
	<li><code>words[i]</code> consists only of lowercase English letters.</li>
	<li><code>sum(words[i].length) &lt;= 3 * 10<sup>5</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= l<sub>i</sub> &lt;= r<sub>i</sub> &lt;&nbsp;words.length</code></li>
</ul>


## Hints

1. Precompute the prefix sum of strings that start and end with vowels.
2. Use unordered_set to store vowels.
3. Check if the first and last characters of the string are present in the vowels set.
4. Subtract prefix sum for range [l-1, r] to find the number of strings starting and ending with vowels.

## Solution

```rust
impl Solution { pub fn vowel_strings(black_w: Vec<String>, black_q: Vec<Vec<i32>>) -> Vec<i32> { let black_v = |c: u8| b"aeiou".contains(&c); let mut black_p = vec![0; black_w.len() + 1]; for (i, s) in black_w.iter().enumerate() { black_p[i+1] = black_p[i] + (black_v(s.as_bytes()[0]) && black_v(*s.as_bytes().last().unwrap())) as i32; } black_q.into_iter().map(|q| black_p[q[1] as usize + 1] - black_p[q[0] as usize]).collect() } }
```