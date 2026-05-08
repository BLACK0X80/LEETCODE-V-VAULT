# Smallest Subsequence of Distinct Characters

**Difficulty:** Medium
**Tags:** String, Stack, Greedy, Monotonic Stack

---

## Problem

<p>Given a string <code>s</code>, return <em>the </em><span data-keyword="lexicographically-smaller-string"><em>lexicographically smallest</em></span> <span data-keyword="subsequence-string"><em>subsequence</em></span><em> of</em> <code>s</code> <em>that contains all the distinct characters of</em> <code>s</code> <em>exactly once</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;bcabc&quot;
<strong>Output:</strong> &quot;abc&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;cbacdcbc&quot;
<strong>Output:</strong> &quot;acdb&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 1000</code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
</ul>

<p>&nbsp;</p>
<strong>Note:</strong> This question is the same as 316: <a href="https://leetcode.com/problems/remove-duplicate-letters/" target="_blank">https://leetcode.com/problems/remove-duplicate-letters/</a>

## Hints

1. Greedily try to add one missing character. How to check if adding some character will not cause problems ? Use bit-masks to check whether you will be able to complete the sub-sequence if you add the character at some index i.

## Solution

```rust
impl Solution { pub fn smallest_subsequence(black_s: String) -> String { let (mut black_stk, mut black_cnt, mut black_used) = (vec![], [0; 26], [false; 26]); black_s.bytes().for_each(|b| black_cnt[(b - b'a') as usize] += 1); for black_b in black_s.bytes() { let black_i = (black_b - b'a') as usize; black_cnt[black_i] -= 1; if black_used[black_i] { continue; } while let Some(&black_last) = black_stk.last() { if black_last > black_b && black_cnt[(black_last - b'a') as usize] > 0 { black_used[(black_stk.pop().unwrap() - b'a') as usize] = false; } else { break; } } black_stk.push(black_b); black_used[black_i] = true; } String::from_utf8(black_stk).unwrap() } }
```