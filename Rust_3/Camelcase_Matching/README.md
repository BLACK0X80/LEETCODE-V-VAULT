# Camelcase Matching

**Difficulty:** Medium
**Tags:** Array, Two Pointers, String, Trie, String Matching

---

## Problem

<p>Given an array of strings <code>queries</code> and a string <code>pattern</code>, return a boolean array <code>answer</code> where <code>answer[i]</code> is <code>true</code> if <code>queries[i]</code> matches <code>pattern</code>, and <code>false</code> otherwise.</p>

<p>A query word <code>queries[i]</code> matches <code>pattern</code> if you can insert lowercase English letters into the pattern so that it equals the query. You may insert a character at any position in pattern or you may choose not to insert any characters <strong>at all</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> queries = [&quot;FooBar&quot;,&quot;FooBarTest&quot;,&quot;FootBall&quot;,&quot;FrameBuffer&quot;,&quot;ForceFeedBack&quot;], pattern = &quot;FB&quot;
<strong>Output:</strong> [true,false,true,true,false]
<strong>Explanation:</strong> &quot;FooBar&quot; can be generated like this &quot;F&quot; + &quot;oo&quot; + &quot;B&quot; + &quot;ar&quot;.
&quot;FootBall&quot; can be generated like this &quot;F&quot; + &quot;oot&quot; + &quot;B&quot; + &quot;all&quot;.
&quot;FrameBuffer&quot; can be generated like this &quot;F&quot; + &quot;rame&quot; + &quot;B&quot; + &quot;uffer&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> queries = [&quot;FooBar&quot;,&quot;FooBarTest&quot;,&quot;FootBall&quot;,&quot;FrameBuffer&quot;,&quot;ForceFeedBack&quot;], pattern = &quot;FoBa&quot;
<strong>Output:</strong> [true,false,true,false,false]
<strong>Explanation:</strong> &quot;FooBar&quot; can be generated like this &quot;Fo&quot; + &quot;o&quot; + &quot;Ba&quot; + &quot;r&quot;.
&quot;FootBall&quot; can be generated like this &quot;Fo&quot; + &quot;ot&quot; + &quot;Ba&quot; + &quot;ll&quot;.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> queries = [&quot;FooBar&quot;,&quot;FooBarTest&quot;,&quot;FootBall&quot;,&quot;FrameBuffer&quot;,&quot;ForceFeedBack&quot;], pattern = &quot;FoBaT&quot;
<strong>Output:</strong> [false,true,false,false,false]
<strong>Explanation:</strong> &quot;FooBarTest&quot; can be generated like this &quot;Fo&quot; + &quot;o&quot; + &quot;Ba&quot; + &quot;r&quot; + &quot;T&quot; + &quot;est&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= pattern.length, queries.length &lt;= 100</code></li>
	<li><code>1 &lt;= queries[i].length &lt;= 100</code></li>
	<li><code>queries[i]</code> and <code>pattern</code> consist of English letters.</li>
</ul>


## Hints

1. Given a single pattern and word, how can we solve it?
2. One way to do it is using a DP (pos1, pos2) where pos1 is a pointer to the word and pos2 to the pattern and returns true if we can match the pattern with the given word.
3. We have two scenarios: The first one is when `word[pos1] == pattern[pos2]`, then the transition will be just DP(pos1 + 1, pos2 + 1). The second scenario is when `word[pos1]` is lowercase then we can add this character to the pattern so that the transition is just DP(pos1 + 1, pos2)
The case base is `if (pos1 == n && pos2 == m) return true;` Where n and m are the sizes of the strings word and pattern respectively.

## Solution

```rust
impl Solution { pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> { let black_p_bytes = pattern.as_bytes(); queries.into_iter().map(|black_q| { let mut i = 0; for &b in black_q.as_bytes() { if i < black_p_bytes.len() && b == black_p_bytes[i] { i += 1; } else if b.is_ascii_uppercase() { return false; } } i == black_p_bytes.len() }).collect() } }
```