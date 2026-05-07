# Word Squares II

**Difficulty:** Medium
**Tags:** Array, String, Backtracking, Sorting, Enumeration

---

## Problem

<p>You are given a string array <code>words</code>, consisting of <strong>distinct</strong> 4-letter strings, each containing lowercase English letters.</p>

<p>A <strong>word square</strong> consists of 4 <strong>distinct</strong> words: <code>top</code>, <code>left</code>, <code>right</code> and <code>bottom</code>, arranged as follows:</p>

<ul>
	<li><code>top</code> forms the <strong>top row</strong>.</li>
	<li><code>bottom</code> forms the <strong>bottom row</strong>.</li>
	<li><code>left</code> forms the <strong>left column</strong> (top to bottom).</li>
	<li><code>right</code> forms the <strong>right column</strong> (top to bottom).</li>
</ul>

<p>It must satisfy:</p>

<ul>
	<li><code>top[0] == left[0]</code>, <code>top[3] == right[0]</code></li>
	<li><code>bottom[0] == left[3]</code>, <code>bottom[3] == right[3]</code></li>
</ul>

<p>Return all valid <strong>distinct</strong> word squares, sorted in <strong>ascending lexicographic</strong> order by the 4-tuple <code>(top, left, right, bottom)​​​​​​​</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words = [&quot;able&quot;,&quot;area&quot;,&quot;echo&quot;,&quot;also&quot;]</span></p>

<p><strong>Output:</strong> <span class="example-io">[[&quot;able&quot;,&quot;area&quot;,&quot;echo&quot;,&quot;also&quot;],[&quot;area&quot;,&quot;able&quot;,&quot;also&quot;,&quot;echo&quot;]]</span></p>

<p><strong>Explanation:</strong></p>

<p>There are exactly two valid 4-word squares that satisfy all corner constraints:</p>

<ul>
	<li><code>&quot;able&quot;</code> (top), <code>&quot;area&quot;</code> (left), <code>&quot;echo&quot;</code> (right), <code>&quot;also&quot;</code> (bottom)

	<ul>
		<li><code>top[0] == left[0] == &#39;a&#39;</code></li>
		<li><code>top[3] == right[0] == &#39;e&#39;</code></li>
		<li><code>bottom[0] == left[3] == &#39;a&#39;</code></li>
		<li><code>bottom[3] == right[3] == &#39;o&#39;</code></li>
	</ul>
	</li>
	<li><code>&quot;area&quot;</code> (top), <code>&quot;able&quot;</code> (left), <code>&quot;also&quot;</code> (right), <code>&quot;echo&quot;</code> (bottom)
	<ul>
		<li>All corner constraints are satisfied.</li>
	</ul>
	</li>
</ul>

<p>Thus, the answer is <code>[[&quot;able&quot;,&quot;area&quot;,&quot;echo&quot;,&quot;also&quot;],[&quot;area&quot;,&quot;able&quot;,&quot;also&quot;,&quot;echo&quot;]]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words = [&quot;code&quot;,&quot;cafe&quot;,&quot;eden&quot;,&quot;edge&quot;]</span></p>

<p><strong>Output:</strong> <span class="example-io">[]</span></p>

<p><strong>Explanation:</strong></p>

<p>No combination of four words satisfies all four corner constraints. Thus, the answer is empty array <code>[]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>4 &lt;= words.length &lt;= 15</code></li>
	<li><code>words[i].length == 4</code></li>
	<li><code>words[i]</code> consists of only lowercase English letters.</li>
	<li>All <code>words[i]</code> are <strong>distinct</strong>.</li>
</ul>


## Hints

1. Use bruteforce

## Solution

```rust
impl Solution { pub fn word_squares(words: Vec<String>) -> Vec<Vec<String>> { let mut black_ans = Vec::new(); let black_n = words.len(); let black_b: Vec<&[u8]> = words.iter().map(|black_w| black_w.as_bytes()).collect(); for black_i in 0..black_n { for black_j in 0..black_n { if black_i == black_j || black_b[black_i][0] != black_b[black_j][0] { continue; } for black_k in 0..black_n { if black_k == black_i || black_k == black_j || black_b[black_i][3] != black_b[black_k][0] { continue; } for black_l in 0..black_n { if black_l == black_i || black_l == black_j || black_l == black_k || black_b[black_l][0] != black_b[black_j][3] || black_b[black_l][3] != black_b[black_k][3] { continue; } black_ans.push(vec![words[black_i].clone(), words[black_j].clone(), words[black_k].clone(), words[black_l].clone()]); } } } } black_ans.sort_unstable(); black_ans } }
```