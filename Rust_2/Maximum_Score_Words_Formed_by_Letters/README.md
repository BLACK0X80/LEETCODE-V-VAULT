# Maximum Score Words Formed by Letters

**Difficulty:** Hard
**Tags:** Array, Hash Table, String, Dynamic Programming, Backtracking, Bit Manipulation, Counting, Bitmask

---

## Problem

<p>Given a list of <code>words</code>, list of&nbsp; single&nbsp;<code>letters</code> (might be repeating)&nbsp;and <code>score</code>&nbsp;of every character.</p>

<p>Return the maximum score of <strong>any</strong> valid set of words formed by using the given letters (<code>words[i]</code> cannot be used two&nbsp;or more times).</p>

<p>It is not necessary to use all characters in <code>letters</code> and each letter can only be used once. Score of letters&nbsp;<code>&#39;a&#39;</code>, <code>&#39;b&#39;</code>, <code>&#39;c&#39;</code>, ... ,<code>&#39;z&#39;</code> is given by&nbsp;<code>score[0]</code>, <code>score[1]</code>, ... , <code>score[25]</code> respectively.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;dog&quot;,&quot;cat&quot;,&quot;dad&quot;,&quot;good&quot;], letters = [&quot;a&quot;,&quot;a&quot;,&quot;c&quot;,&quot;d&quot;,&quot;d&quot;,&quot;d&quot;,&quot;g&quot;,&quot;o&quot;,&quot;o&quot;], score = [1,0,9,5,0,0,3,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0]
<strong>Output:</strong> 23
<strong>Explanation:</strong>
Score  a=1, c=9, d=5, g=3, o=2
Given letters, we can form the words &quot;dad&quot; (5+1+5) and &quot;good&quot; (3+2+2+5) with a score of 23.
Words &quot;dad&quot; and &quot;dog&quot; only get a score of 21.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;xxxz&quot;,&quot;ax&quot;,&quot;bx&quot;,&quot;cx&quot;], letters = [&quot;z&quot;,&quot;a&quot;,&quot;b&quot;,&quot;c&quot;,&quot;x&quot;,&quot;x&quot;,&quot;x&quot;], score = [4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,10]
<strong>Output:</strong> 27
<strong>Explanation:</strong>
Score  a=4, b=4, c=4, x=5, z=10
Given letters, we can form the words &quot;ax&quot; (4+5), &quot;bx&quot; (4+5) and &quot;cx&quot; (4+5) with a score of 27.
Word &quot;xxxz&quot; only get a score of 25.</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;leetcode&quot;], letters = [&quot;l&quot;,&quot;e&quot;,&quot;t&quot;,&quot;c&quot;,&quot;o&quot;,&quot;d&quot;], score = [0,0,1,1,1,0,0,0,0,0,0,1,0,0,1,0,0,0,0,1,0,0,0,0,0,0]
<strong>Output:</strong> 0
<strong>Explanation:</strong>
Letter &quot;e&quot; can only be used once.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 14</code></li>
	<li><code>1 &lt;= words[i].length &lt;= 15</code></li>
	<li><code>1 &lt;= letters.length &lt;= 100</code></li>
	<li><code>letters[i].length == 1</code></li>
	<li><code>score.length ==&nbsp;26</code></li>
	<li><code>0 &lt;= score[i] &lt;= 10</code></li>
	<li><code>words[i]</code>, <code>letters[i]</code>&nbsp;contains only lower case English letters.</li>
</ul>


## Hints

1. Note that words.length is small. This means you can iterate over every subset of words (2^N).

## Solution

```rust
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut avail = [0i32; 26];
        for c in letters { avail[(c as u8 - b'a') as usize] += 1; }

        let words: Vec<([i32;26], i32)> = words.iter().map(|w| {
            let mut freq = [0i32; 26];
            let mut sc = 0;
            for b in w.bytes() {
                let i = (b - b'a') as usize;
                freq[i] += 1;
                sc += score[i];
            }
            (freq, sc)
        }).collect();

        let mut best = 0;
        for mask in 0u32..1<<words.len() {
            let mut freq = [0i32; 26];
            let mut sc = 0;
            let mut valid = true;
            for i in 0..words.len() {
                if mask & (1<<i) == 0 { continue; }
                sc += words[i].1;
                for j in 0..26 {
                    freq[j] += words[i].0[j];
                    if freq[j] > avail[j] { valid = false; }
                }
            }
            if valid { best = best.max(sc); }
        }
        best
    }
}
```