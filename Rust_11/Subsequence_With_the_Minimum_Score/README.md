# Subsequence With the Minimum Score

**Difficulty:** Hard
**Tags:** Two Pointers, String, Binary Search

---

## Problem

<p>You are given two strings <code>s</code> and <code>t</code>.</p>

<p>You are allowed to remove any number of characters from the string <code>t</code>.</p>

<p>The score of the string is <code>0</code> if no characters are removed from the string <code>t</code>, otherwise:</p>

<ul>
	<li>Let <code>left</code> be the minimum index among all removed characters.</li>
	<li>Let <code>right</code> be the maximum index among all removed characters.</li>
</ul>

<p>Then the score of the string is <code>right - left + 1</code>.</p>

<p>Return <em>the minimum possible score to make </em><code>t</code><em>&nbsp;a subsequence of </em><code>s</code><em>.</em></p>

<p>A <strong>subsequence</strong> of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., <code>&quot;ace&quot;</code> is a subsequence of <code>&quot;<u>a</u>b<u>c</u>d<u>e</u>&quot;</code> while <code>&quot;aec&quot;</code> is not).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abacaba&quot;, t = &quot;bzaa&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> In this example, we remove the character &quot;z&quot; at index 1 (0-indexed).
The string t becomes &quot;baa&quot; which is a subsequence of the string &quot;abacaba&quot; and the score is 1 - 1 + 1 = 1.
It can be proven that 1 is the minimum score that we can achieve.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;cde&quot;, t = &quot;xyz&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> In this example, we remove characters &quot;x&quot;, &quot;y&quot; and &quot;z&quot; at indices 0, 1, and 2 (0-indexed).
The string t becomes &quot;&quot; which is a subsequence of the string &quot;cde&quot; and the score is 2 - 0 + 1 = 3.
It can be proven that 3 is the minimum score that we can achieve.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length, t.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> and <code>t</code> consist of only lowercase English letters.</li>
</ul>


## Hints

1. Maintain two pointers: i and j. We need to perform a similar operation: while t[0:i] + t[j:n] is not a subsequence of the string s, increase j.
2. We can check the condition greedily. Create the array leftmost[i] which denotes minimum index k, such that in prefix s[0:k] exists subsequence t[0:i]. Similarly, we define rightmost[i].
3. If leftmost[i] < rightmost[j] then t[0:i] + t[j:n] is the subsequence of s.

## Solution

```rust
impl Solution {
    pub fn minimum_score(black_s: String, black_t: String) -> i32 {
        let black_s_bytes = black_s.as_bytes();
        let black_t_bytes = black_t.as_bytes();
        let black_n = black_s_bytes.len();
        let black_m = black_t_bytes.len();

        let mut black_suffix = vec![-1; black_m];
        let mut black_j = (black_m as i32) - 1;
        for black_i in (0..black_n).rev() {
            if black_j >= 0 && black_s_bytes[black_i] == black_t_bytes[black_j as usize] {
                black_suffix[black_j as usize] = black_i as i32;
                black_j -= 1;
            }
        }

        let mut black_ans = black_m as i32;
        
        let mut black_right_ptr = 0;
        while black_right_ptr < black_m && black_suffix[black_right_ptr] == -1 {
            black_right_ptr += 1;
        }
        black_ans = black_ans.min(black_right_ptr as i32);

        let mut black_j = 0;
        let mut black_suffix_idx = black_right_ptr;

        for black_i in 0..black_n {
            if black_j < black_m && black_s_bytes[black_i] == black_t_bytes[black_j] {
                black_j += 1;
                
                while black_suffix_idx < black_m && (black_suffix[black_suffix_idx] <= black_i as i32 || black_suffix_idx < black_j) {
                    black_suffix_idx += 1;
                }
                
                let black_current_score = if black_suffix_idx >= black_m {
                    (black_m - black_j) as i32
                } else {
                    (black_suffix_idx - black_j) as i32
                };
                
                black_ans = black_ans.min(black_current_score.max(0));
            }
        }

        black_ans.max(0)
    }
}
```