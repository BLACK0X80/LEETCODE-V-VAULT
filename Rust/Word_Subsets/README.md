# Word Subsets

**Difficulty:** Medium
**Tags:** Array, Hash Table, String

---

## Problem

<p>You are given two string arrays <code>words1</code> and <code>words2</code>.</p>

<p>A string <code>b</code> is a <strong>subset</strong> of string <code>a</code> if every letter in <code>b</code> occurs in <code>a</code> including multiplicity.</p>

<ul>
	<li>For example, <code>&quot;wrr&quot;</code> is a subset of <code>&quot;warrior&quot;</code> but is not a subset of <code>&quot;world&quot;</code>.</li>
</ul>

<p>A string <code>a</code> from <code>words1</code> is <strong>universal</strong> if for every string <code>b</code> in <code>words2</code>, <code>b</code> is a subset of <code>a</code>.</p>

<p>Return an array of all the <strong>universal</strong> strings in <code>words1</code>. You may return the answer in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words1 = [&quot;amazon&quot;,&quot;apple&quot;,&quot;facebook&quot;,&quot;google&quot;,&quot;leetcode&quot;], words2 = [&quot;e&quot;,&quot;o&quot;]</span></p>

<p><strong>Output:</strong> <span class="example-io">[&quot;facebook&quot;,&quot;google&quot;,&quot;leetcode&quot;]</span></p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words1 = [&quot;amazon&quot;,&quot;apple&quot;,&quot;facebook&quot;,&quot;google&quot;,&quot;leetcode&quot;], words2 = [&quot;lc&quot;,&quot;eo&quot;]</span></p>

<p><strong>Output:</strong> <span class="example-io">[&quot;leetcode&quot;]</span></p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words1 = [&quot;acaac&quot;,&quot;cccbb&quot;,&quot;aacbb&quot;,&quot;caacc&quot;,&quot;bcbbb&quot;], words2 = [&quot;c&quot;,&quot;cc&quot;,&quot;b&quot;]</span></p>

<p><strong>Output:</strong> <span class="example-io">[&quot;cccbb&quot;]</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= words1.length, words2.length &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= words1[i].length, words2[i].length &lt;= 10</code></li>
	<li><code>words1[i]</code> and <code>words2[i]</code> consist only of lowercase English letters.</li>
	<li>All the strings of <code>words1</code> are <strong>unique</strong>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> { let mut black_max_f = [0; 26]; for black_b in words2 { let mut black_tmp = [0; 26]; black_b.bytes().for_each(|b| black_tmp[(b - b'a') as usize] += 1); (0..26).for_each(|i| black_max_f[i] = black_max_f[i].max(black_tmp[i])); } words1.into_iter().filter(|black_a| { let mut black_f = [0; 26]; black_a.bytes().for_each(|b| black_f[(b - b'a') as usize] += 1); (0..26).all(|i| black_f[i] >= black_max_f[i]) }).collect() } }
```