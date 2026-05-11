# Count the Number of Special Characters II

**Difficulty:** Medium
**Tags:** Hash Table, String

---

## Problem

<p>You are given a string <code>word</code>. A letter&nbsp;<code>c</code> is called <strong>special</strong> if it appears <strong>both</strong> in lowercase and uppercase in <code>word</code>, and <strong>every</strong> lowercase occurrence of <code>c</code> appears before the <strong>first</strong> uppercase occurrence of <code>c</code>.</p>

<p>Return the number of<em> </em><strong>special</strong> letters<em> </em>in<em> </em><code>word</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word = &quot;aaAbcBC&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The special characters are <code>&#39;a&#39;</code>, <code>&#39;b&#39;</code>, and <code>&#39;c&#39;</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word = &quot;abc&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>There are no special characters in <code>word</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word = &quot;AbBCab&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>There are no special characters in <code>word</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= word.length &lt;= 2 * 10<sup>5</sup></code></li>
	<li><code>word</code> consists of only lowercase and uppercase English letters.</li>
</ul>


## Hints

1. For each character <code>c</code>, store the first occurrence of its uppercase and the last occurrence of its lowercase.

## Solution

```rust
impl Solution { pub fn number_of_special_chars(black_word: String) -> i32 { let (mut black_last_low, mut black_first_up) = ([None; 26], [None; 26]); for (i, b) in black_word.bytes().enumerate() { if b >= b'a' { black_last_low[(b - b'a') as usize] = Some(i); } else { let idx = (b - b'A') as usize; if black_first_up[idx].is_none() { black_first_up[idx] = Some(i); } } } (0..26).filter(|&i| { match (black_last_low[i], black_first_up[i]) { (Some(l), Some(u)) => l < u, _ => false } }).count() as i32 } }
```