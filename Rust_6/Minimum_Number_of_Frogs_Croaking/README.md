# Minimum Number of Frogs Croaking

**Difficulty:** Medium
**Tags:** String, Counting

---

## Problem

<p>You are given the string <code>croakOfFrogs</code>, which represents a combination of the string <code>&quot;croak&quot;</code> from different frogs, that is, multiple frogs can croak at the same time, so multiple <code>&quot;croak&quot;</code> are mixed.</p>

<p><em>Return the minimum number of </em>different<em> frogs to finish all the croaks in the given string.</em></p>

<p>A valid <code>&quot;croak&quot;</code> means a frog is printing five letters <code>&#39;c&#39;</code>, <code>&#39;r&#39;</code>, <code>&#39;o&#39;</code>, <code>&#39;a&#39;</code>, and <code>&#39;k&#39;</code> <strong>sequentially</strong>. The frogs have to print all five letters to finish a croak. If the given string is not a combination of a valid <code>&quot;croak&quot;</code> return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> croakOfFrogs = &quot;croakcroak&quot;
<strong>Output:</strong> 1 
<strong>Explanation:</strong> One frog yelling &quot;croak<strong>&quot;</strong> twice.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> croakOfFrogs = &quot;crcoakroak&quot;
<strong>Output:</strong> 2 
<strong>Explanation:</strong> The minimum number of frogs is two. 
The first frog could yell &quot;<strong>cr</strong>c<strong>oak</strong>roak&quot;.
The second frog could yell later &quot;cr<strong>c</strong>oak<strong>roak</strong>&quot;.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> croakOfFrogs = &quot;croakcrook&quot;
<strong>Output:</strong> -1
<strong>Explanation:</strong> The given string is an invalid combination of &quot;croak<strong>&quot;</strong> from different frogs.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= croakOfFrogs.length &lt;= 10<sup>5</sup></code></li>
	<li><code>croakOfFrogs</code> is either <code>&#39;c&#39;</code>, <code>&#39;r&#39;</code>, <code>&#39;o&#39;</code>, <code>&#39;a&#39;</code>, or <code>&#39;k&#39;</code>.</li>
</ul>


## Hints

1. keep the frequency of all characters from "croak" using a hashmap.
2. For each character in the given string, greedily match it to a possible "croak".

## Solution

```rust
impl Solution { pub fn min_number_of_frogs(black_s: String) -> i32 { let (mut black_cnt, mut black_frogs, mut black_max) = ([0; 5], 0, 0); for &b in black_s.as_bytes() { let black_idx = match b { b'c' => 0, b'r' => 1, b'o' => 2, b'a' => 3, b'k' => 4, _ => return -1 }; black_cnt[black_idx] += 1; if black_idx == 0 { black_frogs += 1; black_max = black_max.max(black_frogs); } else { if black_cnt[black_idx - 1] < black_cnt[black_idx] { return -1; } if black_idx == 4 { black_frogs -= 1; } } } if black_frogs == 0 && black_cnt.windows(2).all(|w| w[0] == w[1]) { black_max } else { -1 } } }
```