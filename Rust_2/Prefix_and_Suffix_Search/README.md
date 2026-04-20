# Prefix and Suffix Search

**Difficulty:** Hard
**Tags:** Array, Hash Table, String, Design, Trie

---

## Problem

<p>Design a special dictionary that searches the words in it by a prefix and a suffix.</p>

<p>Implement the <code>WordFilter</code> class:</p>

<ul>
	<li><code>WordFilter(string[] words)</code> Initializes the object with the <code>words</code> in the dictionary.</li>
	<li><code>f(string pref, string suff)</code> Returns <em>the index of the word in the dictionary,</em> which has the prefix <code>pref</code> and the suffix <code>suff</code>. If there is more than one valid index, return <strong>the largest</strong> of them. If there is no such word in the dictionary, return <code>-1</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;WordFilter&quot;, &quot;f&quot;]
[[[&quot;apple&quot;]], [&quot;a&quot;, &quot;e&quot;]]
<strong>Output</strong>
[null, 0]
<strong>Explanation</strong>
WordFilter wordFilter = new WordFilter([&quot;apple&quot;]);
wordFilter.f(&quot;a&quot;, &quot;e&quot;); // return 0, because the word at index 0 has prefix = &quot;a&quot; and suffix = &quot;e&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= words[i].length &lt;= 7</code></li>
	<li><code>1 &lt;= pref.length, suff.length &lt;= 7</code></li>
	<li><code>words[i]</code>, <code>pref</code> and <code>suff</code> consist of lowercase English letters only.</li>
	<li>At most <code>10<sup>4</sup></code> calls will be made to the function <code>f</code>.</li>
</ul>


## Hints

1. Take "apple" as an example, we will insert add "apple{apple", "pple{apple", "ple{apple", "le{apple", "e{apple", "{apple" into the Trie Tree.
2. If the query is: prefix = "app", suffix = "le", we can find it by querying our trie for
"le { app".
3. We use '{' because in ASCii Table, '{' is next to 'z', so we just need to create new TrieNode[27] instead of 26. Also, compared with traditional Trie, we add the attribute weight in class TrieNode.
You can still choose any different character.

## Solution

```rust
use std::collections::HashMap;

struct WordFilter {
    map: HashMap<(String, String), i32>,
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut m = HashMap::new();
        for (i, w) in words.iter().enumerate() {
            for a in 0..=w.len() {
                for b in 0..=w.len() {
                    m.insert((w[..a].into(), w[w.len()-b..].into()), i as i32);
                }
            }
        }
        Self { map: m }
    }
    
    fn f(&self, pref: String, suff: String) -> i32 {
        self.map.get(&(pref, suff)).copied().unwrap_or(-1)
    }
}
```