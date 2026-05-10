# Groups of Special-Equivalent Strings

**Difficulty:** Medium
**Tags:** Array, Hash Table, String, Sorting

---

## Problem

<p>You are given an array of strings of the same length <code>words</code>.</p>

<p>In one <strong>move</strong>, you can swap any two even indexed characters or any two odd indexed characters of a string <code>words[i]</code>.</p>

<p>Two strings <code>words[i]</code> and <code>words[j]</code> are <strong>special-equivalent</strong> if after any number of moves, <code>words[i] == words[j]</code>.</p>

<ul>
	<li>For example, <code>words[i] = &quot;zzxy&quot;</code> and <code>words[j] = &quot;xyzz&quot;</code> are <strong>special-equivalent</strong> because we may make the moves <code>&quot;zzxy&quot; -&gt; &quot;xzzy&quot; -&gt; &quot;xyzz&quot;</code>.</li>
</ul>

<p>A <strong>group of special-equivalent strings</strong> from <code>words</code> is a non-empty subset of words such that:</p>

<ul>
	<li>Every pair of strings in the group are special equivalent, and</li>
	<li>The group is the largest size possible (i.e., there is not a string <code>words[i]</code> not in the group such that <code>words[i]</code> is special-equivalent to every string in the group).</li>
</ul>

<p>Return <em>the number of <strong>groups of special-equivalent strings</strong> from </em><code>words</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;abcd&quot;,&quot;cdab&quot;,&quot;cbad&quot;,&quot;xyzz&quot;,&quot;zzxy&quot;,&quot;zzyx&quot;]
<strong>Output:</strong> 3
<strong>Explanation:</strong> 
One group is [&quot;abcd&quot;, &quot;cdab&quot;, &quot;cbad&quot;], since they are all pairwise special equivalent, and none of the other strings is all pairwise special equivalent to these.
The other two groups are [&quot;xyzz&quot;, &quot;zzxy&quot;] and [&quot;zzyx&quot;].
Note that in particular, &quot;zzxy&quot; is not special equivalent to &quot;zzyx&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;abc&quot;,&quot;acb&quot;,&quot;bac&quot;,&quot;bca&quot;,&quot;cab&quot;,&quot;cba&quot;]
<strong>Output:</strong> 3
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 1000</code></li>
	<li><code>1 &lt;= words[i].length &lt;= 20</code></li>
	<li><code>words[i]</code> consist of lowercase English letters.</li>
	<li>All the strings are of the same length.</li>
</ul>



## Solution

```rust
impl Solution { pub fn num_special_equiv_groups(black_words: Vec<String>) -> i32 { black_words.into_iter().map(|w| { let (mut black_even, mut black_odd) = ([0; 26], [0; 26]); for (i, b) in w.bytes().enumerate() { if i % 2 == 0 { black_even[(b - b'a') as usize] += 1; } else { black_odd[(b - b'a') as usize] += 1; } } (black_even, black_odd) }).collect::<std::collections::HashSet<_>>().len() as i32 } }
```