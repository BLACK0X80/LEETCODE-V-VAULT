# Smallest String With Swaps

**Difficulty:** Medium
**Tags:** Array, Hash Table, String, Depth-First Search, Breadth-First Search, Union-Find, Sorting

---

## Problem

<p>You are given a string <code>s</code>, and an array of pairs of indices in the string&nbsp;<code>pairs</code>&nbsp;where&nbsp;<code>pairs[i] =&nbsp;[a, b]</code>&nbsp;indicates 2 indices(0-indexed) of the string.</p>

<p>You can&nbsp;swap the characters at any pair of indices in the given&nbsp;<code>pairs</code>&nbsp;<strong>any number of times</strong>.</p>

<p>Return the&nbsp;lexicographically smallest string that <code>s</code>&nbsp;can be changed to after using the swaps.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;dcab&quot;, pairs = [[0,3],[1,2]]
<strong>Output:</strong> &quot;bacd&quot;
<strong>Explaination:</strong> 
Swap s[0] and s[3], s = &quot;bcad&quot;
Swap s[1] and s[2], s = &quot;bacd&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;dcab&quot;, pairs = [[0,3],[1,2],[0,2]]
<strong>Output:</strong> &quot;abcd&quot;
<strong>Explaination: </strong>
Swap s[0] and s[3], s = &quot;bcad&quot;
Swap s[0] and s[2], s = &quot;acbd&quot;
Swap s[1] and s[2], s = &quot;abcd&quot;</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;cba&quot;, pairs = [[0,1],[1,2]]
<strong>Output:</strong> &quot;abc&quot;
<strong>Explaination: </strong>
Swap s[0] and s[1], s = &quot;bca&quot;
Swap s[1] and s[2], s = &quot;bac&quot;
Swap s[0] and s[1], s = &quot;abc&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10^5</code></li>
	<li><code>0 &lt;= pairs.length &lt;= 10^5</code></li>
	<li><code>0 &lt;= pairs[i][0], pairs[i][1] &lt;&nbsp;s.length</code></li>
	<li><code>s</code>&nbsp;only contains lower case English letters.</li>
</ul>


## Hints

1. Think of it as a graph problem.
2. Consider the pairs as connected nodes in the graph, what can you do with a connected component of indices ?
3. We can sort each connected component alone to get the lexicographically minimum string.

## Solution

```rust
impl Solution { pub fn smallest_string_with_swaps(black_s: String, black_p: Vec<Vec<i32>>) -> String { let black_n = black_s.len(); let mut black_f: Vec<usize> = (0..black_n).collect(); fn black_find(i: usize, f: &mut Vec<usize>) -> usize { if f[i] == i { i } else { f[i] = black_find(f[i], f); f[i] } } for p in black_p { let (r1, r2) = (black_find(p[0] as usize, &mut black_f), black_find(p[1] as usize, &mut black_f)); if r1 != r2 { black_f[r1] = r2; } } let mut black_g = std::collections::HashMap::new(); let black_b = black_s.as_bytes(); for i in 0..black_n { black_g.entry(black_find(i, &mut black_f)).or_insert((vec![], vec![])).0.push(i); black_g.get_mut(&black_find(i, &mut black_f)).unwrap().1.push(black_b[i]); } let mut black_res = vec![0u8; black_n]; for (_, (mut idx, mut chr)) in black_g { idx.sort(); chr.sort(); for i in 0..idx.len() { black_res[idx[i]] = chr[i]; } } String::from_utf8(black_res).unwrap() } }
```