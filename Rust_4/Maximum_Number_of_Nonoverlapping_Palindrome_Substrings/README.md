# Maximum Number of Non-overlapping Palindrome Substrings

**Difficulty:** Hard
**Tags:** Two Pointers, String, Dynamic Programming, Greedy

---

## Problem

<p>You are given a string <code>s</code> and a <strong>positive</strong> integer <code>k</code>.</p>

<p>Select a set of <strong>non-overlapping</strong> substrings from the string <code>s</code> that satisfy the following conditions:</p>

<ul>
	<li>The <strong>length</strong> of each substring is <strong>at least</strong> <code>k</code>.</li>
	<li>Each substring is a <strong>palindrome</strong>.</li>
</ul>

<p>Return <em>the <strong>maximum</strong> number of substrings in an optimal selection</em>.</p>

<p>A <strong>substring</strong> is a contiguous sequence of characters within a string.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abaccdbbd&quot;, k = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> We can select the substrings underlined in s = &quot;<u><strong>aba</strong></u>cc<u><strong>dbbd</strong></u>&quot;. Both &quot;aba&quot; and &quot;dbbd&quot; are palindromes and have a length of at least k = 3.
It can be shown that we cannot find a selection with more than two valid substrings.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;adbcda&quot;, k = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no palindrome substring of length at least 2 in the string.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= s.length &lt;= 2000</code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
</ul>


## Hints

1. Try to use dynamic programming to solve the problem.
2. let dp[i] be the answer for the prefix s[0…i].
3. The final answer to the problem will be dp[n-1]. How do you compute this dp?

## Solution

```rust
impl Solution {
    pub fn max_palindromes(black: String, k: i32) -> i32 {
        let b = black.as_bytes();
        let bn = b.len();
        let k = k as usize;
        let mut track = vec![vec![false; bn]; bn];

        for i in 0..bn {
            track[i][i] = true;
            let (mut bl, mut blk) = (i as i32 - 1, i as i32 + 1);
            while bl >= 0 && blk < bn as i32 && b[bl as usize] == b[blk as usize] {
                track[bl as usize][blk as usize] = true;
                bl -= 1; blk += 1;
            }
            if i + 1 < bn && b[i] == b[i+1] {
                track[i][i+1] = true;
                let (mut bl, mut blk) = (i as i32 - 1, i as i32 + 2);
                while bl >= 0 && blk < bn as i32 && b[bl as usize] == b[blk as usize] {
                    track[bl as usize][blk as usize] = true;
                    bl -= 1; blk += 1;
                }
            }
        }

        let mut black0 = vec![0i32; bn + 1];
        for i in (0..bn).rev() {
            black0[i] = black0[i+1];
            for j in k..=bn {
                if i + j - 1 < bn && track[i][i+j-1] {
                    black0[i] = black0[i].max(1 + black0[i+j]);
                }
            }
        }
        black0[0]
    }
}
```