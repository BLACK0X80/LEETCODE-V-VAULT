# K-Similar Strings

**Difficulty:** Hard
**Tags:** Hash Table, String, Breadth-First Search

---

## Problem

<p>Strings <code>s1</code> and <code>s2</code> are <code>k</code><strong>-similar</strong> (for some non-negative integer <code>k</code>) if we can swap the positions of two letters in <code>s1</code> exactly <code>k</code> times so that the resulting string equals <code>s2</code>.</p>

<p>Given two anagrams <code>s1</code> and <code>s2</code>, return the smallest <code>k</code> for which <code>s1</code> and <code>s2</code> are <code>k</code><strong>-similar</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;ab&quot;, s2 = &quot;ba&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> The two string are 1-similar because we can use one swap to change s1 to s2: &quot;ab&quot; --&gt; &quot;ba&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;abc&quot;, s2 = &quot;bca&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> The two strings are 2-similar because we can use two swaps to change s1 to s2: &quot;abc&quot; --&gt; &quot;bac&quot; --&gt; &quot;bca&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s1.length &lt;= 20</code></li>
	<li><code>s2.length == s1.length</code></li>
	<li><code>s1</code> and <code>s2</code> contain only lowercase letters from the set <code>{&#39;a&#39;, &#39;b&#39;, &#39;c&#39;, &#39;d&#39;, &#39;e&#39;, &#39;f&#39;}</code>.</li>
	<li><code>s2</code> is an anagram of <code>s1</code>.</li>
</ul>



## Solution

```rust
use std::collections::{VecDeque, HashSet};
impl Solution {
    pub fn k_similarity(black_s1: String, black_s2: String) -> i32 {
        let (black_a, black_b) = (black_s1.into_bytes(), black_s2.into_bytes());
        let (mut black_q, mut black_v) = (VecDeque::from([(black_a.clone(), 0)]), HashSet::from([black_a]));
        while let Some((mut black_curr, black_dist)) = black_q.pop_front() {
            if black_curr == black_b { return black_dist; }
            let mut black_i = 0;
            while black_curr[black_i] == black_b[black_i] { black_i += 1; }
            for black_j in black_i + 1..black_curr.len() {
                if black_curr[black_j] == black_b[black_i] && black_curr[black_j] != black_b[black_j] {
                    black_curr.swap(black_i, black_j);
                    if black_v.insert(black_curr.clone()) { black_q.push_back((black_curr.clone(), black_dist + 1)); }
                    black_curr.swap(black_i, black_j);
                }
            }
        }
        0
    }
}
```