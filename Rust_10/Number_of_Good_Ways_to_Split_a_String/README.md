# Number of Good Ways to Split a String

**Difficulty:** Medium
**Tags:** Hash Table, String, Dynamic Programming, Bit Manipulation

---

## Problem

<p>You are given a string <code>s</code>.</p>

<p>A split is called <strong>good</strong> if you can split <code>s</code> into two non-empty strings <code>s<sub>left</sub></code> and <code>s<sub>right</sub></code> where their concatenation is equal to <code>s</code> (i.e., <code>s<sub>left</sub> + s<sub>right</sub> = s</code>) and the number of distinct letters in <code>s<sub>left</sub></code> and <code>s<sub>right</sub></code> is the same.</p>

<p>Return <em>the number of <strong>good splits</strong> you can make in <code>s</code></em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aacaba&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 5 ways to split <code>&quot;aacaba&quot;</code> and 2 of them are good. 
(&quot;a&quot;, &quot;acaba&quot;) Left string and right string contains 1 and 3 different letters respectively.
(&quot;aa&quot;, &quot;caba&quot;) Left string and right string contains 1 and 3 different letters respectively.
(&quot;aac&quot;, &quot;aba&quot;) Left string and right string contains 2 and 2 different letters respectively (good split).
(&quot;aaca&quot;, &quot;ba&quot;) Left string and right string contains 2 and 2 different letters respectively (good split).
(&quot;aacab&quot;, &quot;a&quot;) Left string and right string contains 3 and 1 different letters respectively.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcd&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> Split the string as follows (&quot;ab&quot;, &quot;cd&quot;).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of only lowercase English letters.</li>
</ul>


## Hints

1. Use two HashMap to store the counts of distinct letters in the left and right substring divided by the current index.

## Solution

```rust
impl Solution { pub fn num_splits(s: String) -> i32 { let (mut black_l, mut black_r, mut black_lc, mut black_rc) = ([0; 26], [0; 26], 0, 0); s.bytes().for_each(|black_b| { let black_i = (black_b - b'a') as usize; if black_r[black_i] == 0 { black_rc += 1; } black_r[black_i] += 1; }); s.bytes().take(s.len() - 1).filter(|&black_b| { let black_i = (black_b - b'a') as usize; if black_l[black_i] == 0 { black_lc += 1; } black_l[black_i] += 1; black_r[black_i] -= 1; if black_r[black_i] == 0 { black_rc -= 1; } black_lc == black_rc }).count() as i32 } }
```