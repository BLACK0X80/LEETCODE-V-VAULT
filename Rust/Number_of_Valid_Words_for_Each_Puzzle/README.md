# Number of Valid Words for Each Puzzle

**Difficulty:** Hard
**Tags:** Array, Hash Table, String, Bit Manipulation, Trie

---

## Problem

With respect to a given <code>puzzle</code> string, a <code>word</code> is <em>valid</em> if both the following conditions are satisfied:
<ul>
	<li><code>word</code> contains the first letter of <code>puzzle</code>.</li>
	<li>For each letter in <code>word</code>, that letter is in <code>puzzle</code>.
	<ul>
		<li>For example, if the puzzle is <code>&quot;abcdefg&quot;</code>, then valid words are <code>&quot;faced&quot;</code>, <code>&quot;cabbage&quot;</code>, and <code>&quot;baggage&quot;</code>, while</li>
		<li>invalid words are <code>&quot;beefed&quot;</code> (does not include <code>&#39;a&#39;</code>) and <code>&quot;based&quot;</code> (includes <code>&#39;s&#39;</code> which is not in the puzzle).</li>
	</ul>
	</li>
</ul>
Return <em>an array </em><code>answer</code><em>, where </em><code>answer[i]</code><em> is the number of words in the given word list </em><code>words</code><em> that is valid with respect to the puzzle </em><code>puzzles[i]</code>.
<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;aaaa&quot;,&quot;asas&quot;,&quot;able&quot;,&quot;ability&quot;,&quot;actt&quot;,&quot;actor&quot;,&quot;access&quot;], puzzles = [&quot;aboveyz&quot;,&quot;abrodyz&quot;,&quot;abslute&quot;,&quot;absoryz&quot;,&quot;actresz&quot;,&quot;gaswxyz&quot;]
<strong>Output:</strong> [1,1,3,2,4,0]
<strong>Explanation:</strong> 
1 valid word for &quot;aboveyz&quot; : &quot;aaaa&quot; 
1 valid word for &quot;abrodyz&quot; : &quot;aaaa&quot;
3 valid words for &quot;abslute&quot; : &quot;aaaa&quot;, &quot;asas&quot;, &quot;able&quot;
2 valid words for &quot;absoryz&quot; : &quot;aaaa&quot;, &quot;asas&quot;
4 valid words for &quot;actresz&quot; : &quot;aaaa&quot;, &quot;asas&quot;, &quot;actt&quot;, &quot;access&quot;
There are no valid words for &quot;gaswxyz&quot; cause none of the words in the list contains letter &#39;g&#39;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;apple&quot;,&quot;pleas&quot;,&quot;please&quot;], puzzles = [&quot;aelwxyz&quot;,&quot;aelpxyz&quot;,&quot;aelpsxy&quot;,&quot;saelpxy&quot;,&quot;xaelpsy&quot;]
<strong>Output:</strong> [0,1,3,2,0]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 10<sup>5</sup></code></li>
	<li><code>4 &lt;= words[i].length &lt;= 50</code></li>
	<li><code>1 &lt;= puzzles.length &lt;= 10<sup>4</sup></code></li>
	<li><code>puzzles[i].length == 7</code></li>
	<li><code>words[i]</code> and <code>puzzles[i]</code> consist of lowercase English letters.</li>
	<li>Each <code>puzzles[i] </code>does not contain repeated characters.</li>
</ul>


## Hints

1. Exploit the fact that the length of the puzzle is only 7.
2. Use bit-masks to represent the word and puzzle strings.
3. For each puzzle, count the number of words whose bit-mask is a sub-mask of the puzzle's bit-mask.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut black_freq: HashMap<u32, i32> = HashMap::new();
        for w in &words {
            let black_mask = w.bytes().fold(0u32, |m, b| m | (1 << (b - b'a')));
            *black_freq.entry(black_mask).or_insert(0) += 1;
        }
        puzzles.iter().map(|p| {
            let black_pmask = p.bytes().fold(0u32, |m, b| m | (1 << (b - b'a')));
            let black_first = 1u32 << (p.as_bytes()[0] - b'a');
            let mut black_sub = black_pmask;
            let mut black_ans = 0i32;
            loop {
                if black_sub & black_first != 0 {
                    black_ans += black_freq.get(&black_sub).unwrap_or(&0);
                }
                if black_sub == 0 { break; }
                black_sub = (black_sub - 1) & black_pmask;
            }
            black_ans
        }).collect()
    }
}
```