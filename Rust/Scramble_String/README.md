# Scramble String

**Difficulty:** Hard
**Tags:** String, Dynamic Programming

---

## Problem

<p>We can scramble a string s to get a string t using the following algorithm:</p>

<ol>
	<li>If the length of the string is 1, stop.</li>
	<li>If the length of the string is &gt; 1, do the following:
	<ul>
		<li>Split the string into two non-empty substrings at a random index, i.e., if the string is <code>s</code>, divide it to <code>x</code> and <code>y</code> where <code>s = x + y</code>.</li>
		<li><strong>Randomly</strong>&nbsp;decide to swap the two substrings or to keep them in the same order. i.e., after this step, <code>s</code> may become <code>s = x + y</code> or <code>s = y + x</code>.</li>
		<li>Apply step 1 recursively on each of the two substrings <code>x</code> and <code>y</code>.</li>
	</ul>
	</li>
</ol>

<p>Given two strings <code>s1</code> and <code>s2</code> of <strong>the same length</strong>, return <code>true</code> if <code>s2</code> is a scrambled string of <code>s1</code>, otherwise, return <code>false</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;great&quot;, s2 = &quot;rgeat&quot;
<strong>Output:</strong> true
<strong>Explanation:</strong> One possible scenario applied on s1 is:
&quot;great&quot; --&gt; &quot;gr/eat&quot; // divide at random index.
&quot;gr/eat&quot; --&gt; &quot;gr/eat&quot; // random decision is not to swap the two substrings and keep them in order.
&quot;gr/eat&quot; --&gt; &quot;g/r / e/at&quot; // apply the same algorithm recursively on both substrings. divide at random index each of them.
&quot;g/r / e/at&quot; --&gt; &quot;r/g / e/at&quot; // random decision was to swap the first substring and to keep the second substring in the same order.
&quot;r/g / e/at&quot; --&gt; &quot;r/g / e/ a/t&quot; // again apply the algorithm recursively, divide &quot;at&quot; to &quot;a/t&quot;.
&quot;r/g / e/ a/t&quot; --&gt; &quot;r/g / e/ a/t&quot; // random decision is to keep both substrings in the same order.
The algorithm stops now, and the result string is &quot;rgeat&quot; which is s2.
As one possible scenario led s1 to be scrambled to s2, we return true.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;abcde&quot;, s2 = &quot;caebd&quot;
<strong>Output:</strong> false
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s1 = &quot;a&quot;, s2 = &quot;a&quot;
<strong>Output:</strong> true
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>s1.length == s2.length</code></li>
	<li><code>1 &lt;= s1.length &lt;= 30</code></li>
	<li><code>s1</code> and <code>s2</code> consist of lowercase English letters.</li>
</ul>



## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let mut memo = HashMap::new();
        Self::solve(s1.as_bytes(), s2.as_bytes(), &mut memo)
    }

    fn solve(s1: &[u8], s2: &[u8], memo: &mut HashMap<(Vec<u8>, Vec<u8>), bool>) -> bool {
        if s1 == s2 { return true; }

        let mut freq = [0i32; 26];
        for (&a, &b) in s1.iter().zip(s2.iter()) {
            freq[(a - b'a') as usize] += 1;
            freq[(b - b'a') as usize] -= 1;
        }
        if freq.iter().any(|&x| x != 0) { return false; }

        let key = (s1.to_vec(), s2.to_vec());
        if let Some(&res) = memo.get(&key) { return res; }

        let n = s1.len();
        for i in 1..n {
            let res =
                (Self::solve(&s1[..i], &s2[..i], memo) && Self::solve(&s1[i..], &s2[i..], memo))
             || (Self::solve(&s1[..i], &s2[n-i..], memo) && Self::solve(&s1[i..], &s2[..n-i], memo));
            if res {
                memo.insert(key, true);
                return true;
            }
        }

        memo.insert(key, false);
        false
    }
}
```