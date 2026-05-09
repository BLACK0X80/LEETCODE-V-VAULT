# Find Longest Special Substring That Occurs Thrice II

**Difficulty:** Medium
**Tags:** Hash Table, String, Binary Search, Sliding Window, Counting

---

## Problem

<p>You are given a string <code>s</code> that consists of lowercase English letters.</p>

<p>A string is called <strong>special</strong> if it is made up of only a single character. For example, the string <code>&quot;abc&quot;</code> is not special, whereas the strings <code>&quot;ddd&quot;</code>, <code>&quot;zz&quot;</code>, and <code>&quot;f&quot;</code> are special.</p>

<p>Return <em>the length of the <strong>longest special substring</strong> of </em><code>s</code> <em>which occurs <strong>at least thrice</strong></em>, <em>or </em><code>-1</code><em> if no special substring occurs at least thrice</em>.</p>

<p>A <strong>substring</strong> is a contiguous <strong>non-empty</strong> sequence of characters within a string.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aaaa&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> The longest special substring which occurs thrice is &quot;aa&quot;: substrings &quot;<u><strong>aa</strong></u>aa&quot;, &quot;a<u><strong>aa</strong></u>a&quot;, and &quot;aa<u><strong>aa</strong></u>&quot;.
It can be shown that the maximum length achievable is 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcdef&quot;
<strong>Output:</strong> -1
<strong>Explanation:</strong> There exists no special substring which occurs at least thrice. Hence return -1.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcaba&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> The longest special substring which occurs thrice is &quot;a&quot;: substrings &quot;<u><strong>a</strong></u>bcaba&quot;, &quot;abc<u><strong>a</strong></u>ba&quot;, and &quot;abcab<u><strong>a</strong></u>&quot;.
It can be shown that the maximum length achievable is 1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= s.length &lt;= 5 * 10<sup>5</sup></code></li>
	<li><code>s</code> consists of only lowercase English letters.</li>
</ul>


## Hints

1. Let <code>len[i]</code> be the length of the longest special string ending with <code>s[i]</code>.
2. If <code>i > 0</code> and <code>s[i] == s[i - 1]</code>, <code>len[i] = len[i - 1] + 1</code>. Otherwise <code>len[i] == 1</code>.
3. Group all the <code>len[i]</code> by <code>s[i]</code>. We have at most <code>26</code> groups.
4. The maximum value of the third largest <code>len[i]</code> in each group is the answer.
5. We only need to maintain the top three values for each group. You can use sorting, heap, or brute-force comparison to find the third largest value in each group.

## Solution

```rust
impl Solution { pub fn maximum_length(black_s: String) -> i32 { let (black_v, mut black_counts) = (black_s.as_bytes(), vec![vec![]; 26]); let (mut black_i, mut black_res) = (0, -1); while black_i < black_v.len() { let mut black_j = black_i; while black_j < black_v.len() && black_v[black_j] == black_v[black_i] { black_j += 1; } black_counts[(black_v[black_i] - b'a') as usize].push((black_j - black_i) as i32); black_i = black_j; } for mut black_c in black_counts { if black_c.is_empty() { continue; } black_c.sort_unstable_by(|a, b| b.cmp(a)); let black_l = black_c.len(); let mut black_cur = -1; if black_c[0] >= 3 { black_cur = black_cur.max(black_c[0] - 2); } if black_l >= 2 { if black_c[0] == black_c[1] { black_cur = black_cur.max(black_c[0] - 1); } else { black_cur = black_cur.max(black_c[1].min(black_c[0] - 1)); } } if black_l >= 3 { black_cur = black_cur.max(black_c[2]); } if black_cur > 0 { black_res = black_res.max(black_cur); } } black_res } }
```