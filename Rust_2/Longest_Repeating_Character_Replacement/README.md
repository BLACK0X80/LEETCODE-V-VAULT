# Longest Repeating Character Replacement

**Difficulty:** Medium
**Tags:** Hash Table, String, Sliding Window

---

## Problem

<p>You are given a string <code>s</code> and an integer <code>k</code>. You can choose any character of the string and change it to any other uppercase English character. You can perform this operation at most <code>k</code> times.</p>

<p>Return <em>the length of the longest substring containing the same letter you can get after performing the above operations</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;ABAB&quot;, k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> Replace the two &#39;A&#39;s with two &#39;B&#39;s or vice versa.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;AABABBA&quot;, k = 1
<strong>Output:</strong> 4
<strong>Explanation:</strong> Replace the one &#39;A&#39; in the middle with &#39;B&#39; and form &quot;AABBBBA&quot;.
The substring &quot;BBBB&quot; has the longest repeating letters, which is 4.
There may exists other ways to achieve this answer too.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of only uppercase English letters.</li>
	<li><code>0 &lt;= k &lt;= s.length</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut black = [0i32; 26];
        let mut left = 0;
        let mut max_f = 0;
        let mut ans = 0;
        for right in 0..s.len() {
            let idx = (s[right] - b'A') as usize;
            black[idx] += 1;
            if black[idx] > max_f { max_f = black[idx]; }
            if (right - left + 1) as i32 - max_f > k {
                black[(s[left] - b'A') as usize] -= 1;
                left += 1;
            }
            ans = ans.max(right - left + 1);
        }
        ans as i32
    }
}
```