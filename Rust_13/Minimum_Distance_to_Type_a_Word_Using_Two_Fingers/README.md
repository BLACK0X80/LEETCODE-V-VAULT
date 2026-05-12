# Minimum Distance to Type a Word Using Two Fingers

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<img alt="" src="https://assets.leetcode.com/uploads/2020/01/02/leetcode_keyboard.png" style="width: 349px; height: 209px;" />
<p>You have a keyboard layout as shown above in the <strong>X-Y</strong> plane, where each English uppercase letter is located at some coordinate.</p>

<ul>
	<li>For example, the letter <code>&#39;A&#39;</code> is located at coordinate <code>(0, 0)</code>, the letter <code>&#39;B&#39;</code> is located at coordinate <code>(0, 1)</code>, the letter <code>&#39;P&#39;</code> is located at coordinate <code>(2, 3)</code> and the letter <code>&#39;Z&#39;</code> is located at coordinate <code>(4, 1)</code>.</li>
</ul>

<p>Given the string <code>word</code>, return <em>the minimum total <strong>distance</strong> to type such string using only two fingers</em>.</p>

<p>The <strong>distance</strong> between coordinates <code>(x<sub>1</sub>, y<sub>1</sub>)</code> and <code>(x<sub>2</sub>, y<sub>2</sub>)</code> is <code>|x<sub>1</sub> - x<sub>2</sub>| + |y<sub>1</sub> - y<sub>2</sub>|</code>.</p>

<p><strong>Note</strong> that the initial positions of your two fingers are considered free so do not count towards your total distance, also your two fingers do not have to start at the first letter or the first two letters.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> word = &quot;CAKE&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> Using two fingers, one optimal way to type &quot;CAKE&quot; is: 
Finger 1 on letter &#39;C&#39; -&gt; cost = 0 
Finger 1 on letter &#39;A&#39; -&gt; cost = Distance from letter &#39;C&#39; to letter &#39;A&#39; = 2 
Finger 2 on letter &#39;K&#39; -&gt; cost = 0 
Finger 2 on letter &#39;E&#39; -&gt; cost = Distance from letter &#39;K&#39; to letter &#39;E&#39; = 1 
Total distance = 3
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> word = &quot;HAPPY&quot;
<strong>Output:</strong> 6
<strong>Explanation:</strong> Using two fingers, one optimal way to type &quot;HAPPY&quot; is:
Finger 1 on letter &#39;H&#39; -&gt; cost = 0
Finger 1 on letter &#39;A&#39; -&gt; cost = Distance from letter &#39;H&#39; to letter &#39;A&#39; = 2
Finger 2 on letter &#39;P&#39; -&gt; cost = 0
Finger 2 on letter &#39;P&#39; -&gt; cost = Distance from letter &#39;P&#39; to letter &#39;P&#39; = 0
Finger 1 on letter &#39;Y&#39; -&gt; cost = Distance from letter &#39;A&#39; to letter &#39;Y&#39; = 4
Total distance = 6
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= word.length &lt;= 300</code></li>
	<li><code>word</code> consists of uppercase English letters.</li>
</ul>


## Hints

1. Use dynamic programming.
2. dp[i][j][k]: smallest movements when you have one finger on i-th char and the other one on j-th char already having written k first characters from word.

## Solution

```rust
impl Solution {
    pub fn minimum_distance(black_word: String) -> i32 {
        let black_bytes = black_word.as_bytes();
        let black_n = black_bytes.len();
        let mut black_dp = vec![vec![vec![1000000; 27]; 27]; black_n + 1];

        black_dp[0][26][26] = 0;

        for black_i in 0..black_n {
            let black_target = (black_bytes[black_i] - b'A') as usize;
            for black_f1 in 0..27 {
                for black_f2 in 0..27 {
                    let black_current_dist = black_dp[black_i][black_f1][black_f2];
                    if black_current_dist == 1000000 { continue; }

                    let black_cost1 = if black_f1 == 26 { 0 } else {
                        (black_f1 as i32 / 6 - black_target as i32 / 6).abs() + (black_f1 as i32 % 6 - black_target as i32 % 6).abs()
                    };
                    black_dp[black_i + 1][black_target][black_f2] = black_dp[black_i + 1][black_target][black_f2].min(black_current_dist + black_cost1);

                    let black_cost2 = if black_f2 == 26 { 0 } else {
                        (black_f2 as i32 / 6 - black_target as i32 / 6).abs() + (black_f2 as i32 % 6 - black_target as i32 % 6).abs()
                    };
                    black_dp[black_i + 1][black_f1][black_target] = black_dp[black_i + 1][black_f1][black_target].min(black_current_dist + black_cost2);
                }
            }
        }

        let mut black_ans = 1000000;
        for black_i in 0..27 {
            for black_j in 0..27 {
                black_ans = black_ans.min(black_dp[black_n][black_i][black_j]);
            }
        }
        black_ans
    }
}
```