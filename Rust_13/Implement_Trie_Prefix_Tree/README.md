# Implement Trie (Prefix Tree)

**Difficulty:** Medium
**Tags:** Hash Table, String, Design, Trie

---

## Problem

<p>A <a href="https://en.wikipedia.org/wiki/Trie" target="_blank"><strong>trie</strong></a> (pronounced as &quot;try&quot;) or <strong>prefix tree</strong> is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. There are various applications of this data structure, such as autocomplete and spellchecker.</p>

<p>Implement the Trie class:</p>

<ul>
	<li><code>Trie()</code> Initializes the trie object.</li>
	<li><code>void insert(String word)</code> Inserts the string <code>word</code> into the trie.</li>
	<li><code>boolean search(String word)</code> Returns <code>true</code> if the string <code>word</code> is in the trie (i.e., was inserted before), and <code>false</code> otherwise.</li>
	<li><code>boolean startsWith(String prefix)</code> Returns <code>true</code> if there is a previously inserted string <code>word</code> that has the prefix <code>prefix</code>, and <code>false</code> otherwise.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;Trie&quot;, &quot;insert&quot;, &quot;search&quot;, &quot;search&quot;, &quot;startsWith&quot;, &quot;insert&quot;, &quot;search&quot;]
[[], [&quot;apple&quot;], [&quot;apple&quot;], [&quot;app&quot;], [&quot;app&quot;], [&quot;app&quot;], [&quot;app&quot;]]
<strong>Output</strong>
[null, null, true, false, true, null, true]

<strong>Explanation</strong>
Trie trie = new Trie();
trie.insert(&quot;apple&quot;);
trie.search(&quot;apple&quot;);   // return True
trie.search(&quot;app&quot;);     // return False
trie.startsWith(&quot;app&quot;); // return True
trie.insert(&quot;app&quot;);
trie.search(&quot;app&quot;);     // return True
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= word.length, prefix.length &lt;= 2000</code></li>
	<li><code>word</code> and <code>prefix</code> consist only of lowercase English letters.</li>
	<li>At most <code>3 * 10<sup>4</sup></code> calls <strong>in total</strong> will be made to <code>insert</code>, <code>search</code>, and <code>startsWith</code>.</li>
</ul>



## Solution

```rust
#[derive(Default)] struct Trie { black_c: [Option<Box<Trie>>; 26], black_e: bool }
impl Trie {
    fn new() -> Self { Default::default() }
    fn insert(&mut self, black_w: String) { let mut black_curr = self; for &black_b in black_w.as_bytes() { black_curr = black_curr.black_c[(black_b - b'a') as usize].get_or_insert_with(Default::default); } black_curr.black_e = true; }
    fn search(&self, black_w: String) -> bool { self.black_f(black_w).map_or(false, |black_n| black_n.black_e) }
    fn starts_with(&self, black_p: String) -> bool { self.black_f(black_p).is_some() }
    fn black_f(&self, black_s: String) -> Option<&Trie> { let mut black_curr = self; for &black_b in black_s.as_bytes() { black_curr = black_curr.black_c[(black_b - b'a') as usize].as_deref()?; } Some(black_curr) }
}
```