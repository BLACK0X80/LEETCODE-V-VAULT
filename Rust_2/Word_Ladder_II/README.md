# Word Ladder II

**Difficulty:** Hard
**Tags:** Hash Table, String, Backtracking, Breadth-First Search

---

## Problem

<p>A <strong>transformation sequence</strong> from word <code>beginWord</code> to word <code>endWord</code> using a dictionary <code>wordList</code> is a sequence of words <code>beginWord -&gt; s<sub>1</sub> -&gt; s<sub>2</sub> -&gt; ... -&gt; s<sub>k</sub></code> such that:</p>

<ul>
	<li>Every adjacent pair of words differs by a single letter.</li>
	<li>Every <code>s<sub>i</sub></code> for <code>1 &lt;= i &lt;= k</code> is in <code>wordList</code>. Note that <code>beginWord</code> does not need to be in <code>wordList</code>.</li>
	<li><code>s<sub>k</sub> == endWord</code></li>
</ul>

<p>Given two words, <code>beginWord</code> and <code>endWord</code>, and a dictionary <code>wordList</code>, return <em>all the <strong>shortest transformation sequences</strong> from</em> <code>beginWord</code> <em>to</em> <code>endWord</code><em>, or an empty list if no such sequence exists. Each sequence should be returned as a list of the words </em><code>[beginWord, s<sub>1</sub>, s<sub>2</sub>, ..., s<sub>k</sub>]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> beginWord = &quot;hit&quot;, endWord = &quot;cog&quot;, wordList = [&quot;hot&quot;,&quot;dot&quot;,&quot;dog&quot;,&quot;lot&quot;,&quot;log&quot;,&quot;cog&quot;]
<strong>Output:</strong> [[&quot;hit&quot;,&quot;hot&quot;,&quot;dot&quot;,&quot;dog&quot;,&quot;cog&quot;],[&quot;hit&quot;,&quot;hot&quot;,&quot;lot&quot;,&quot;log&quot;,&quot;cog&quot;]]
<strong>Explanation:</strong>&nbsp;There are 2 shortest transformation sequences:
&quot;hit&quot; -&gt; &quot;hot&quot; -&gt; &quot;dot&quot; -&gt; &quot;dog&quot; -&gt; &quot;cog&quot;
&quot;hit&quot; -&gt; &quot;hot&quot; -&gt; &quot;lot&quot; -&gt; &quot;log&quot; -&gt; &quot;cog&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> beginWord = &quot;hit&quot;, endWord = &quot;cog&quot;, wordList = [&quot;hot&quot;,&quot;dot&quot;,&quot;dog&quot;,&quot;lot&quot;,&quot;log&quot;]
<strong>Output:</strong> []
<strong>Explanation:</strong> The endWord &quot;cog&quot; is not in wordList, therefore there is no valid transformation sequence.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= beginWord.length &lt;= 5</code></li>
	<li><code>endWord.length == beginWord.length</code></li>
	<li><code>1 &lt;= wordList.length &lt;= 500</code></li>
	<li><code>wordList[i].length == beginWord.length</code></li>
	<li><code>beginWord</code>, <code>endWord</code>, and <code>wordList[i]</code> consist of lowercase English letters.</li>
	<li><code>beginWord != endWord</code></li>
	<li>All the words in <code>wordList</code> are <strong>unique</strong>.</li>
	<li>The <strong>sum</strong> of all shortest transformation sequences does not exceed <code>10<sup>5</sup></code>.</li>
</ul>



## Solution

```rust
use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn find_ladders(begin: String, end: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        let word_set: HashSet<String> = word_list.into_iter().collect();
        if !word_set.contains(&end) { return vec![]; }

        let mut parents: HashMap<String, Vec<String>> = HashMap::new();
        let mut layer: HashSet<String> = HashSet::new();
        layer.insert(begin.clone());
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(begin.clone());
        let mut found = false;

        'bfs: while !layer.is_empty() {
            let mut next: HashSet<String> = HashSet::new();
            for word in &layer {
                let mut chars: Vec<char> = word.chars().collect();
                for i in 0..chars.len() {
                    let orig = chars[i];
                    for c in 'a'..='z' {
                        if c == orig { continue; }
                        chars[i] = c;
                        let new_word: String = chars.iter().collect();
                        if word_set.contains(&new_word) && !visited.contains(&new_word) {
                            next.insert(new_word.clone());
                            parents.entry(new_word.clone()).or_default().push(word.clone());
                            if new_word == end { found = true; }
                        }
                        chars[i] = orig;
                    }
                }
            }
            if found { break 'bfs; }
            for w in &next { visited.insert(w.clone()); }
            layer = next;
        }

        if !found { return vec![]; }

        let mut result = vec![];
        let mut path = vec![end.clone()];
        Self::dfs(&end, &begin, &parents, &mut path, &mut result);
        result
    }

    fn dfs(word: &str, begin: &str, parents: &HashMap<String, Vec<String>>, path: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
        if word == begin {
            let mut p = path.clone();
            p.reverse();
            result.push(p);
            return;
        }
        if let Some(pars) = parents.get(word) {
            for par in pars {
                path.push(par.clone());
                Self::dfs(par, begin, parents, path, result);
                path.pop();
            }
        }
    }
}
```