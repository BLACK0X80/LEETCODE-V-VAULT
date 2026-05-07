# Minimum Cost Good Caption

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>You are given a string <code>caption</code> of length <code>n</code>. A <strong>good</strong> caption is a string where <strong>every</strong> character appears in groups of <strong>at least 3</strong> consecutive occurrences.</p>

<p>For example:</p>

<ul>
	<li><code>&quot;aaabbb&quot;</code> and <code>&quot;aaaaccc&quot;</code> are <strong>good</strong> captions.</li>
	<li><code>&quot;aabbb&quot;</code> and <code>&quot;ccccd&quot;</code> are <strong>not</strong> good captions.</li>
</ul>

<p>You can perform the following operation <strong>any</strong> number of times:</p>

<p>Choose an index <code>i</code> (where <code>0 &lt;= i &lt; n</code>) and change the character at that index to either:</p>

<ul>
	<li>The character immediately <strong>before</strong> it in the alphabet (if <code>caption[i] != &#39;a&#39;</code>).</li>
	<li>The character immediately <strong>after</strong> it in the alphabet (if <code>caption[i] != &#39;z&#39;</code>).</li>
</ul>

<p>Your task is to convert the given <code>caption</code> into a <strong>good</strong> caption using the <strong>minimum</strong> number of operations, and return it. If there are <strong>multiple</strong> possible good captions, return the <strong><span data-keyword="lexicographically-smaller-string">lexicographically smallest</span></strong> one among them. If it is <strong>impossible</strong> to create a good caption, return an empty string <code>&quot;&quot;</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">caption = &quot;cdcd&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;cccc&quot;</span></p>

<p><strong>Explanation:</strong></p>

<p>It can be shown that the given caption cannot be transformed into a good caption with fewer than 2 operations. The possible good captions that can be created using exactly 2 operations are:</p>

<ul>
	<li><code>&quot;dddd&quot;</code>: Change <code>caption[0]</code> and <code>caption[2]</code> to their next character <code>&#39;d&#39;</code>.</li>
	<li><code>&quot;cccc&quot;</code>: Change <code>caption[1]</code> and <code>caption[3]</code> to their previous character <code>&#39;c&#39;</code>.</li>
</ul>

<p>Since <code>&quot;cccc&quot;</code> is lexicographically smaller than <code>&quot;dddd&quot;</code>, return <code>&quot;cccc&quot;</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">caption = &quot;aca&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;aaa&quot;</span></p>

<p><strong>Explanation:</strong></p>

<p>It can be proven that the given caption requires at least 2 operations to be transformed into a good caption. The only good caption that can be obtained with exactly 2 operations is as follows:</p>

<ul>
	<li>Operation 1: Change <code>caption[1]</code> to <code>&#39;b&#39;</code>. <code>caption = &quot;aba&quot;</code>.</li>
	<li>Operation 2: Change <code>caption[1]</code> to <code>&#39;a&#39;</code>. <code>caption = &quot;aaa&quot;</code>.</li>
</ul>

<p>Thus, return <code>&quot;aaa&quot;</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">caption = &quot;bc&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;&quot;</span></p>

<p><strong>Explanation:</strong></p>

<p>It can be shown that the given caption cannot be converted to a good caption by using any number of operations.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= caption.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>caption</code> consists only of lowercase English letters.</li>
</ul>


## Hints

1. Construct a DP table and try all possible characters at every index.
2. Choose characters greedily to get the lexicographically smallest caption.

## Solution

```rust
impl Solution {
    pub fn min_cost_good_caption(black_caption: String) -> String {
        let black_n = black_caption.len();
        if black_n < 3 { return "".to_string(); }
        let black_b = black_caption.as_bytes();
        let mut black_dp = vec![vec![vec![(i32::MAX / 2, 0u8); 4]; 26]; black_n + 1];

        for black_last in 0..26 {
            for black_cnt in 0..4 {
                black_dp[black_n][black_last][black_cnt] = if black_cnt >= 3 { (0, 0) } else { (-1, 0) };
            }
        }

        for black_i in (0..black_n).rev() {
            let black_orig = (black_b[black_i] - b'a') as i32;
            for black_last in 0..26 {
                for black_cnt in 0..4 {
                    let mut black_best_cost = i32::MAX / 2;
                    let mut black_best_char = 0u8;

                    for black_ch in 0..26 {
                        if black_cnt >= 1 && black_cnt < 3 && black_ch != black_last { continue; }
                        
                        let black_next_cnt = if black_ch == black_last { (black_cnt + 1).min(3) } else { 1 };
                        let black_res = black_dp[black_i + 1][black_ch as usize][black_next_cnt as usize];

                        if black_res.0 != -1 {
                            let black_cost = (black_orig - black_ch as i32).abs() + black_res.0;
                            if black_cost < black_best_cost {
                                black_best_cost = black_cost;
                                black_best_char = black_ch as u8;
                            }
                        }
                    }
                    black_dp[black_i][black_last][black_cnt] = if black_best_cost >= i32::MAX / 2 { (-1, 0) } else { (black_best_cost, black_best_char) };
                }
            }
        }

        let mut black_ans = String::with_capacity(black_n);
        let mut black_curr_char = 0usize;
        let mut black_curr_cnt = 0usize;
        
        let mut black_min_start_cost = i32::MAX / 2;
        for black_ch in 0..26 {
            let (black_c, _) = black_dp[0][black_ch][0];
            if black_c != -1 && black_c < black_min_start_cost {
                black_min_start_cost = black_c;
                black_curr_char = black_ch;
            }
        }

        for black_i in 0..black_n {
            let (_, black_ch) = black_dp[black_i][black_curr_char][black_curr_cnt];
            black_ans.push((b'a' + black_ch) as char);
            black_curr_cnt = if black_i > 0 && black_ch == (black_ans.as_bytes()[black_i-1] - b'a') { (black_curr_cnt + 1).min(3) } else { 1 };
            black_curr_char = black_ch as usize;
        }

        black_ans
    }
}
```