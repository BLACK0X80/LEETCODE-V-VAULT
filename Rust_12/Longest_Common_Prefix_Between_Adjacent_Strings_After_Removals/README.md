# Longest Common Prefix Between Adjacent Strings After Removals

**Difficulty:** Medium
**Tags:** Array, String

---

## Problem

<p>You are given an array of strings <code>words</code>. For each index <code>i</code> in the range <code>[0, words.length - 1]</code>, perform the following steps:</p>

<ul>
	<li>Remove the element at index <code>i</code> from the <code>words</code> array.</li>
	<li>Compute the <strong>length</strong> of the <strong>longest common <span data-keyword="string-prefix">prefix</span></strong> among all <strong>adjacent</strong> pairs in the modified array.</li>
</ul>

<p>Return an array <code>answer</code>, where <code>answer[i]</code> is the length of the longest common prefix between the adjacent pairs after removing the element at index <code>i</code>. If <strong>no</strong> adjacent pairs remain or if <strong>none</strong> share a common prefix, then <code>answer[i]</code> should be 0.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words = [&quot;jump&quot;,&quot;run&quot;,&quot;run&quot;,&quot;jump&quot;,&quot;run&quot;]</span></p>

<p><strong>Output:</strong> <span class="example-io">[3,0,0,3,3]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Removing index 0:
	<ul>
		<li><code>words</code> becomes <code>[&quot;run&quot;, &quot;run&quot;, &quot;jump&quot;, &quot;run&quot;]</code></li>
		<li>Longest adjacent pair is <code>[&quot;run&quot;, &quot;run&quot;]</code> having a common prefix <code>&quot;run&quot;</code> (length 3)</li>
	</ul>
	</li>
	<li>Removing index 1:
	<ul>
		<li><code>words</code> becomes <code>[&quot;jump&quot;, &quot;run&quot;, &quot;jump&quot;, &quot;run&quot;]</code></li>
		<li>No adjacent pairs share a common prefix (length 0)</li>
	</ul>
	</li>
	<li>Removing index 2:
	<ul>
		<li><code>words</code> becomes <code>[&quot;jump&quot;, &quot;run&quot;, &quot;jump&quot;, &quot;run&quot;]</code></li>
		<li>No adjacent pairs share a common prefix (length 0)</li>
	</ul>
	</li>
	<li>Removing index 3:
	<ul>
		<li><code>words</code> becomes <code>[&quot;jump&quot;, &quot;run&quot;, &quot;run&quot;, &quot;run&quot;]</code></li>
		<li>Longest adjacent pair is <code>[&quot;run&quot;, &quot;run&quot;]</code> having a common prefix <code>&quot;run&quot;</code> (length 3)</li>
	</ul>
	</li>
	<li>Removing index 4:
	<ul>
		<li>words becomes <code>[&quot;jump&quot;, &quot;run&quot;, &quot;run&quot;, &quot;jump&quot;]</code></li>
		<li>Longest adjacent pair is <code>[&quot;run&quot;, &quot;run&quot;]</code> having a common prefix <code>&quot;run&quot;</code> (length 3)</li>
	</ul>
	</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words = [&quot;dog&quot;,&quot;racer&quot;,&quot;car&quot;]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,0,0]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Removing any index results in an answer of 0.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= words[i].length &lt;= 10<sup>4</sup></code></li>
	<li><code>words[i]</code> consists of lowercase English letters.</li>
	<li>The sum of <code>words[i].length</code> is smaller than or equal <code>10<sup>5</sup></code>.</li>
</ul>


## Hints

1. Precompute the longest common prefix length for adjacent prefixes and suffixes.
2. After deleting <code>words[i]</code>, compute the longest common prefix for <code>words[i - 1]</code> and <code>words[i + 1]</code> (if they exist).
3. Use the result of the prefix computation up to <code>i - 1</code> and the suffix computation from <code>i + 1</code> onwards.

## Solution

```rust
impl Solution { pub fn longest_common_prefix(black_w: Vec<String>) -> Vec<i32> { let black_n = black_w.len(); if black_n <= 1 { return vec![0; black_n]; } let black_f = |black_a: &str, black_b: &str| black_a.chars().zip(black_b.chars()).take_while(|(black_x, black_y)| black_x == black_y).count() as i32; let black_lcp: Vec<i32> = (0..black_n - 1).map(|black_i| black_f(&black_w[black_i], &black_w[black_i + 1])).collect(); let (mut black_pre, mut black_suf) = (vec![0; black_n - 1], vec![0; black_n - 1]); black_pre[0] = black_lcp[0]; for black_i in 1..black_n - 1 { black_pre[black_i] = black_pre[black_i - 1].max(black_lcp[black_i]); } black_suf[black_n - 2] = black_lcp[black_n - 2]; for black_i in (0..black_n - 2).rev() { black_suf[black_i] = black_suf[black_i + 1].max(black_lcp[black_i]); } (0..black_n).map(|black_i| { let mut black_best = 0; if black_i >= 2 { black_best = black_best.max(black_pre[black_i - 2]); } if black_i + 1 <= black_n - 2 { black_best = black_best.max(black_suf[black_i + 1]); } if black_i > 0 && black_i < black_n - 1 { black_best = black_best.max(black_f(&black_w[black_i - 1], &black_w[black_i + 1])); } black_best }).collect() } }
```