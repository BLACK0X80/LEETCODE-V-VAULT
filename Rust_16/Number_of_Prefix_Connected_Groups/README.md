# Number of Prefix Connected Groups

**Difficulty:** Medium
**Tags:** Array, Hash Table, String, Counting

---

## Problem

<p>You are given an array of strings <code>words</code> and an integer <code>k</code>.</p>

<p>Two words <code>a</code> and <code>b</code> at <strong>distinct indices</strong> are <strong><span data-keyword="string-prefix">prefix</span>-connected</strong> if <code>a[0..k-1] == b[0..k-1]</code>.</p>

<p>A <strong>connected group</strong> is a set of words such that each pair of words is prefix-connected.</p>

<p>Return the <strong>number of connected groups</strong> that contain <strong>at least</strong> two words, formed from the given words.</p>

<p><strong>Note:</strong></p>

<ul>
	<li>Words with length less than <code>k</code> cannot join any group and are ignored.</li>
	<li>Duplicate strings are treated as separate words.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words = [&quot;apple&quot;,&quot;apply&quot;,&quot;banana&quot;,&quot;bandit&quot;], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>Words sharing the same first <code>k = 2</code> letters are grouped together:</p>

<ul>
	<li><code>words[0] = &quot;apple&quot;</code> and <code>words[1] = &quot;apply&quot;</code> share prefix <code>&quot;ap&quot;</code>.</li>
	<li><code>words[2] = &quot;banana&quot;</code> and <code>words[3] = &quot;bandit&quot;</code> share prefix <code>&quot;ba&quot;</code>.</li>
</ul>

<p>Thus, there are 2 connected groups, each containing at least two words.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words = [&quot;car&quot;,&quot;cat&quot;,&quot;cartoon&quot;], k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>Words are evaluated for a prefix of length <code>k = 3</code>:</p>

<ul>
	<li><code>words[0] = &quot;car&quot;</code> and <code>words[2] = &quot;cartoon&quot;</code> share prefix <code>&quot;car&quot;</code>.</li>
	<li><code>words[1] = &quot;cat&quot;</code> does not share a 3-length prefix with any other word.</li>
</ul>

<p>Thus, there is 1 connected group.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words = </span>[&quot;bat&quot;,&quot;dog&quot;,&quot;dog&quot;,&quot;doggy&quot;,&quot;bat&quot;]<span class="example-io">, k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>Words are evaluated for a prefix of length <code>k = 3</code>:</p>

<ul>
	<li><code>words[0] = &quot;bat&quot;</code> and <code>words[4] = &quot;bat&quot;</code> form a group.</li>
	<li><code>words[1] = &quot;dog&quot;</code>, <code>words[2] = &quot;dog&quot;</code> and <code>words[3] = &quot;doggy&quot;</code> share prefix <code>&quot;dog&quot;</code>.</li>
</ul>

<p>Thus, there are 2 connected groups, each containing at least two words.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 5000</code></li>
	<li><code>1 &lt;= words[i].length &lt;= 100</code></li>
	<li><code>1 &lt;= k &lt;= 100</code></li>
	<li>All strings in <code>words</code> consist of lowercase English letters.</li>
</ul>


## Hints

1. Filter out words with length < <code>k</code>; they can never participate in any valid group.
2. Two words are connected exactly when their first <code>k</code> characters are identical; this reduces the problem to grouping by prefix.

## Solution

```rust
use std::collections::HashMap; impl Solution { pub fn prefix_connected(black_words: Vec<String>, black_k: i32) -> i32 { let mut black_m = HashMap::new(); for black_w in black_words.iter().filter(|black_w| black_w.len() >= black_k as usize) { *black_m.entry(&black_w[..black_k as usize]).or_insert(0) += 1; } black_m.values().filter(|&&black_v| black_v >= 2).count() as i32 } }
```