# Shifting Letters II

**Difficulty:** Medium
**Tags:** Array, String, Prefix Sum

---

## Problem

<p>You are given a string <code>s</code> of lowercase English letters and a 2D integer array <code>shifts</code> where <code>shifts[i] = [start<sub>i</sub>, end<sub>i</sub>, direction<sub>i</sub>]</code>. For every <code>i</code>, <strong>shift</strong> the characters in <code>s</code> from the index <code>start<sub>i</sub></code> to the index <code>end<sub>i</sub></code> (<strong>inclusive</strong>) forward if <code>direction<sub>i</sub> = 1</code>, or shift the characters backward if <code>direction<sub>i</sub> = 0</code>.</p>

<p>Shifting a character <strong>forward</strong> means replacing it with the <strong>next</strong> letter in the alphabet (wrapping around so that <code>&#39;z&#39;</code> becomes <code>&#39;a&#39;</code>). Similarly, shifting a character <strong>backward</strong> means replacing it with the <strong>previous</strong> letter in the alphabet (wrapping around so that <code>&#39;a&#39;</code> becomes <code>&#39;z&#39;</code>).</p>

<p>Return <em>the final string after all such shifts to </em><code>s</code><em> are applied</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abc&quot;, shifts = [[0,1,0],[1,2,1],[0,2,1]]
<strong>Output:</strong> &quot;ace&quot;
<strong>Explanation:</strong> Firstly, shift the characters from index 0 to index 1 backward. Now s = &quot;zac&quot;.
Secondly, shift the characters from index 1 to index 2 forward. Now s = &quot;zbd&quot;.
Finally, shift the characters from index 0 to index 2 forward. Now s = &quot;ace&quot;.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;dztz&quot;, shifts = [[0,0,0],[1,1,1]]
<strong>Output:</strong> &quot;catz&quot;
<strong>Explanation:</strong> Firstly, shift the characters from index 0 to index 0 backward. Now s = &quot;cztz&quot;.
Finally, shift the characters from index 1 to index 1 forward. Now s = &quot;catz&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length, shifts.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>shifts[i].length == 3</code></li>
	<li><code>0 &lt;= start<sub>i</sub> &lt;= end<sub>i</sub> &lt; s.length</code></li>
	<li><code>0 &lt;= direction<sub>i</sub> &lt;= 1</code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
</ul>


## Hints

1. Instead of shifting every character in each shift, could you keep track of which characters are shifted and by how much across all shifts?
2. Try marking the start and ends of each shift, then perform a prefix sum of the shifts.

## Solution

```rust
impl Solution { pub fn shifting_letters(black_s: String, black_shifts: Vec<Vec<i32>>) -> String { let black_n = black_s.len(); let mut black_diff = vec![0i32; black_n + 1]; for black_sh in black_shifts { let (black_st, black_ed, black_dir) = (black_sh[0] as usize, black_sh[1] as usize, if black_sh[2] == 1 { 1 } else { -1 }); black_diff[black_st] += black_dir; black_diff[black_ed + 1] -= black_dir; } let (mut black_cur, black_bytes, mut black_res) = (0, black_s.into_bytes(), vec![0u8; black_n]); for i in 0..black_n { black_cur = (black_cur + black_diff[i]) % 26; let mut black_p = ((black_bytes[i] - b'a') as i32 + black_cur) % 26; if black_p < 0 { black_p += 26; } black_res[i] = (b'a' + black_p as u8); } String::from_utf8(black_res).unwrap() } }
```