# Remove Duplicate Letters

**Difficulty:** Medium
**Tags:** String, Stack, Greedy, Monotonic Stack

---

## Problem

<p>Given a string <code>s</code>, remove duplicate letters so that every letter appears once and only once. You must make sure your result is <span data-keyword="lexicographically-smaller-string"><strong>the smallest in lexicographical order</strong></span> among all possible results.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;bcabc&quot;
<strong>Output:</strong> &quot;abc&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;cbacdcbc&quot;
<strong>Output:</strong> &quot;acdb&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>4</sup></code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
</ul>

<p>&nbsp;</p>
<p><strong>Note:</strong> This question is the same as 1081: <a href="https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/" target="_blank">https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/</a></p>


## Hints

1. Greedily try to add one missing character. How to check if adding some character will not cause problems ? Use bit-masks to check whether you will be able to complete the sub-sequence if you add the character at some index i.

## Solution

```rust
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let s = s.as_bytes();
        let mut freq = [0i32; 26];
        let mut in_stack = [false; 26];
        for &c in s { freq[(c - b'a') as usize] += 1; }
        let mut stack: Vec<u8> = vec![];
        for &c in s {
            let ci = (c - b'a') as usize;
            freq[ci] -= 1;
            if in_stack[ci] { continue; }
            while let Some(&top) = stack.last() {
                let ti = (top - b'a') as usize;
                if top > c && freq[ti] > 0 {
                    stack.pop();
                    in_stack[ti] = false;
                } else { break; }
            }
            stack.push(c);
            in_stack[ci] = true;
        }
        String::from_utf8(stack).unwrap()
    }
}
```