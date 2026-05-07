# Verbal Arithmetic Puzzle

**Difficulty:** Hard
**Tags:** Array, Math, String, Backtracking

---

## Problem

<p>Given an equation, represented by <code>words</code> on the left side and the <code>result</code> on the right side.</p>

<p>You need to check if the equation is solvable under the following rules:</p>

<ul>
	<li>Each character is decoded as one digit (0 - 9).</li>
	<li>No two characters can map to the same digit.</li>
	<li>Each <code>words[i]</code> and <code>result</code> are decoded as one number <strong>without</strong> leading zeros.</li>
	<li>Sum of numbers on the left side (<code>words</code>) will equal to the number on the right side (<code>result</code>).</li>
</ul>

<p>Return <code>true</code> <em>if the equation is solvable, otherwise return</em> <code>false</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;SEND&quot;,&quot;MORE&quot;], result = &quot;MONEY&quot;
<strong>Output:</strong> true
<strong>Explanation:</strong> Map &#39;S&#39;-&gt; 9, &#39;E&#39;-&gt;5, &#39;N&#39;-&gt;6, &#39;D&#39;-&gt;7, &#39;M&#39;-&gt;1, &#39;O&#39;-&gt;0, &#39;R&#39;-&gt;8, &#39;Y&#39;-&gt;&#39;2&#39;
Such that: &quot;SEND&quot; + &quot;MORE&quot; = &quot;MONEY&quot; ,  9567 + 1085 = 10652</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;SIX&quot;,&quot;SEVEN&quot;,&quot;SEVEN&quot;], result = &quot;TWENTY&quot;
<strong>Output:</strong> true
<strong>Explanation:</strong> Map &#39;S&#39;-&gt; 6, &#39;I&#39;-&gt;5, &#39;X&#39;-&gt;0, &#39;E&#39;-&gt;8, &#39;V&#39;-&gt;7, &#39;N&#39;-&gt;2, &#39;T&#39;-&gt;1, &#39;W&#39;-&gt;&#39;3&#39;, &#39;Y&#39;-&gt;4
Such that: &quot;SIX&quot; + &quot;SEVEN&quot; + &quot;SEVEN&quot; = &quot;TWENTY&quot; ,  650 + 68782 + 68782 = 138214</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;LEET&quot;,&quot;CODE&quot;], result = &quot;POINT&quot;
<strong>Output:</strong> false
<strong>Explanation:</strong> There is no possible mapping to satisfy the equation, so we return false.
Note that two different characters cannot map to the same digit.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= words.length &lt;= 5</code></li>
	<li><code>1 &lt;= words[i].length, result.length &lt;= 7</code></li>
	<li><code>words[i], result</code> contain only uppercase English letters.</li>
	<li>The number of different characters used in the expression is at most <code>10</code>.</li>
</ul>


## Hints

1. Use Backtracking and pruning to solve this problem.
2. If you set the values of some digits (from right to left), the other digits will be constrained.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn is_solvable(black_words: Vec<String>, black_result: String) -> bool {
        let mut black_all_words = black_words.clone();
        black_all_words.push(black_result.clone());
        let mut black_chars = std::collections::HashSet::new();
        let mut black_is_start = [false; 26];

        for black_w in &black_all_words {
            if black_w.len() > 1 {
                black_is_start[(black_w.as_bytes()[0] - b'A') as usize] = true;
            }
            for &black_b in black_w.as_bytes() {
                black_chars.insert(black_b);
            }
        }

        let black_char_list: Vec<u8> = black_chars.into_iter().collect();
        let mut black_used_digit = [false; 10];
        let mut black_map = [ -1i8; 26];

        fn black_backtrack(black_idx: usize, black_char_list: &[u8], black_map: &mut [i8; 26], black_used_digit: &mut [bool; 10], black_words: &[String], black_result: &String, black_is_start: &[bool; 26]) -> bool {
            if black_idx == black_char_list.len() {
                let mut black_sum = 0i64;
                for black_w in black_words {
                    let mut black_num = 0i64;
                    for &black_b in black_w.as_bytes() { black_num = black_num * 10 + black_map[(black_b - b'A') as usize] as i64; }
                    black_sum += black_num;
                }
                let mut black_res_num = 0i64;
                for &black_b in black_result.as_bytes() { black_res_num = black_res_num * 10 + black_map[(black_b - b'A') as usize] as i64; }
                return black_sum == black_res_num;
            }

            let black_c = black_char_list[black_idx];
            for black_d in 0..10 {
                if black_used_digit[black_d] || (black_d == 0 && black_is_start[(black_c - b'A') as usize]) { continue; }
                black_used_digit[black_d] = true;
                black_map[(black_c - b'A') as usize] = black_d as i8;
                if black_backtrack(black_idx + 1, black_char_list, black_map, black_used_digit, black_words, black_result, black_is_start) { return true; }
                black_used_digit[black_d] = false;
                black_map[(black_c - b'A') as usize] = -1;
            }
            false
        }

        black_backtrack(0, &black_char_list, &mut black_map, &mut black_used_digit, &black_words, &black_result, &black_is_start)
    }
}
```