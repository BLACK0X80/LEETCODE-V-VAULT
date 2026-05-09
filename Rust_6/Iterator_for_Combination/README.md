# Iterator for Combination

**Difficulty:** Medium
**Tags:** String, Backtracking, Design, Iterator

---

## Problem

<p>Design the <code>CombinationIterator</code> class:</p>

<ul>
	<li><code>CombinationIterator(string characters, int combinationLength)</code> Initializes the object with a string <code>characters</code> of <strong>sorted distinct</strong> lowercase English letters and a number <code>combinationLength</code> as arguments.</li>
	<li><code>next()</code> Returns the next combination of length <code>combinationLength</code> in <strong>lexicographical order</strong>.</li>
	<li><code>hasNext()</code> Returns <code>true</code> if and only if there exists a next combination.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;CombinationIterator&quot;, &quot;next&quot;, &quot;hasNext&quot;, &quot;next&quot;, &quot;hasNext&quot;, &quot;next&quot;, &quot;hasNext&quot;]
[[&quot;abc&quot;, 2], [], [], [], [], [], []]
<strong>Output</strong>
[null, &quot;ab&quot;, true, &quot;ac&quot;, true, &quot;bc&quot;, false]

<strong>Explanation</strong>
CombinationIterator itr = new CombinationIterator(&quot;abc&quot;, 2);
itr.next();    // return &quot;ab&quot;
itr.hasNext(); // return True
itr.next();    // return &quot;ac&quot;
itr.hasNext(); // return True
itr.next();    // return &quot;bc&quot;
itr.hasNext(); // return False
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= combinationLength &lt;= characters.length &lt;= 15</code></li>
	<li>All the characters of <code>characters</code> are <strong>unique</strong>.</li>
	<li>At most <code>10<sup>4</sup></code> calls will be made to <code>next</code> and <code>hasNext</code>.</li>
	<li>It is guaranteed that all calls of the function <code>next</code> are valid.</li>
</ul>


## Hints

1. Generate all combinations as a preprocessing.
2. Use bit masking to generate all the combinations.

## Solution

```rust
struct CombinationIterator { black_combos: Vec<String>, black_ptr: usize } impl CombinationIterator { fn new(black_char: String, black_len: i32) -> Self { let (mut black_res, black_s, black_l) = (vec![], black_char.as_bytes(), black_len as usize); fn black_g(idx: usize, cur: &mut Vec<u8>, s: &[u8], l: usize, res: &mut Vec<String>) { if cur.len() == l { res.push(String::from_utf8(cur.clone()).unwrap()); return; } for i in idx..s.len() { cur.push(s[i]); black_g(i + 1, cur, s, l, res); cur.pop(); } } black_g(0, &mut vec![], black_s, black_l, &mut black_res); Self { black_combos: black_res, black_ptr: 0 } } fn next(&mut self) -> String { self.black_ptr += 1; self.black_combos[self.black_ptr - 1].clone() } fn has_next(&self) -> bool { self.black_ptr < self.black_combos.len() } }
```