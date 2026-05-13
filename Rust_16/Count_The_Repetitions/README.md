# Count The Repetitions

**Difficulty:** Hard
**Tags:** Two Pointers, String, Dynamic Programming

---

## Problem

<p>We define <code>str = [s, n]</code> as the string <code>str</code> which consists of the string <code>s</code> concatenated <code>n</code> times.</p>

<ul>
	<li>For example, <code>str == [&quot;abc&quot;, 3] ==&quot;abcabcabc&quot;</code>.</li>
</ul>

<p>We define that string <code>s1</code> can be obtained from string <code>s2</code> if we can remove some characters from <code>s2</code> such that it becomes <code>s1</code>.</p>

<ul>
	<li>For example, <code>s1 = &quot;abc&quot;</code> can be obtained from <code>s2 = &quot;ab<strong><u>dbe</u></strong>c&quot;</code> based on our definition by removing the bolded underlined characters.</li>
</ul>

<p>You are given two strings <code>s1</code> and <code>s2</code> and two integers <code>n1</code> and <code>n2</code>. You have the two strings <code>str1 = [s1, n1]</code> and <code>str2 = [s2, n2]</code>.</p>

<p>Return <em>the maximum integer </em><code>m</code><em> such that </em><code>str = [str2, m]</code><em> can be obtained from </em><code>str1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> s1 = "acb", n1 = 4, s2 = "ab", n2 = 2
<strong>Output:</strong> 2
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> s1 = "acb", n1 = 1, s2 = "acb", n2 = 1
<strong>Output:</strong> 1
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s1.length, s2.length &lt;= 100</code></li>
	<li><code>s1</code> and <code>s2</code> consist of lowercase English letters.</li>
	<li><code>1 &lt;= n1, n2 &lt;= 10<sup>6</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let black_s1 = s1.as_bytes();
        let black_s2 = s2.as_bytes();
        let black_len1 = black_s1.len();
        let black_len2 = black_s2.len();
        if black_len1 == 0 || black_len2 == 0 || n1 == 0 || n2 == 0 {
            return 0;
        }
        let mut black_counts = vec![0; black_len2 + 1];
        let mut black_indices = vec![0; black_len2 + 1];
        let mut black_count = 0;
        let mut black_index = 0;
        for black_i in 1..=(n1 as usize) {
            for black_j in 0..black_len1 {
                if black_s1[black_j] == black_s2[black_index] {
                    black_index += 1;
                    if black_index == black_len2 {
                        black_index = 0;
                        black_count += 1;
                    }
                }
            }
            black_counts[black_i] = black_count;
            black_indices[black_i] = black_index;
            for black_k in 0..black_i {
                if black_indices[black_k] == black_index {
                    let black_prefix_count = black_counts[black_k];
                    let black_pattern_count = (black_counts[black_i] - black_counts[black_k]) * ((n1 as usize - black_k) / (black_i - black_k));
                    let black_suffix_count = black_counts[black_k + (n1 as usize - black_k) % (black_i - black_k)] - black_counts[black_k];
                    return ((black_prefix_count + black_pattern_count + black_suffix_count) as i32) / n2;
                }
            }
        }
        black_counts[n1 as usize] as i32 / n2
    }
}
```