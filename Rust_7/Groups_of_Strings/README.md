# Groups of Strings

**Difficulty:** Hard
**Tags:** Array, Hash Table, String, Bit Manipulation, Union-Find

---

## Problem

<p>You are given a <strong>0-indexed</strong> array of strings <code>words</code>. Each string consists of <strong>lowercase English letters</strong> only. No letter occurs more than once in any string of <code>words</code>.</p>

<p>Two strings <code>s1</code> and <code>s2</code> are said to be <strong>connected</strong> if the set of letters of <code>s2</code> can be obtained from the set of letters of <code>s1</code> by any <strong>one</strong> of the following operations:</p>

<ul>
	<li>Adding exactly one letter to the set of the letters of <code>s1</code>.</li>
	<li>Deleting exactly one letter from the set of the letters of <code>s1</code>.</li>
	<li>Replacing exactly one letter from the set of the letters of <code>s1</code> with any letter, <strong>including</strong> itself.</li>
</ul>

<p>The array <code>words</code> can be divided into one or more non-intersecting <strong>groups</strong>. A string belongs to a group if any <strong>one</strong> of the following is true:</p>

<ul>
	<li>It is connected to <strong>at least one</strong> other string of the group.</li>
	<li>It is the <strong>only</strong> string present in the group.</li>
</ul>

<p>Note that the strings in <code>words</code> should be grouped in such a manner that a string belonging to a group cannot be connected to a string present in any other group. It can be proved that such an arrangement is always unique.</p>

<p>Return <em>an array</em> <code>ans</code> <em>of size</em> <code>2</code> <em>where:</em></p>

<ul>
	<li><code>ans[0]</code> <em>is the <strong>maximum number</strong> of groups</em> <code>words</code> <em>can be divided into, and</em></li>
	<li><code>ans[1]</code> <em>is the <strong>size of the largest</strong> group</em>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;a&quot;,&quot;b&quot;,&quot;ab&quot;,&quot;cde&quot;]
<strong>Output:</strong> [2,3]
<strong>Explanation:</strong>
- words[0] can be used to obtain words[1] (by replacing &#39;a&#39; with &#39;b&#39;), and words[2] (by adding &#39;b&#39;). So words[0] is connected to words[1] and words[2].
- words[1] can be used to obtain words[0] (by replacing &#39;b&#39; with &#39;a&#39;), and words[2] (by adding &#39;a&#39;). So words[1] is connected to words[0] and words[2].
- words[2] can be used to obtain words[0] (by deleting &#39;b&#39;), and words[1] (by deleting &#39;a&#39;). So words[2] is connected to words[0] and words[1].
- words[3] is not connected to any string in words.
Thus, words can be divided into 2 groups [&quot;a&quot;,&quot;b&quot;,&quot;ab&quot;] and [&quot;cde&quot;]. The size of the largest group is 3.  
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;a&quot;,&quot;ab&quot;,&quot;abc&quot;]
<strong>Output:</strong> [1,3]
<strong>Explanation:</strong>
- words[0] is connected to words[1].
- words[1] is connected to words[0] and words[2].
- words[2] is connected to words[1].
Since all strings are connected to each other, they should be grouped together.
Thus, the size of the largest group is 3.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= words[i].length &lt;= 26</code></li>
	<li><code>words[i]</code> consists of lowercase English letters only.</li>
	<li>No letter occurs more than once in <code>words[i]</code>.</li>
</ul>


## Hints

1. Can we build a graph from words, where there exists an edge between nodes i and j if words[i] and words[j] are connected?
2. The problem now boils down to finding the total number of components and the size of the largest component in the graph.
3. How can we use bit masking to reduce the search space while adding edges to node i?

## Solution

```rust
use std::collections::HashMap;
impl Solution {
    pub fn group_strings(black_w: Vec<String>) -> Vec<i32> {
        let black_n = black_w.len();
        let mut black_parent: Vec<usize> = (0..black_n).collect();
        let mut black_size = vec![1; black_n];
        let mut black_map = HashMap::new();
        fn black_find(i: usize, p: &mut Vec<usize>) -> usize {
            if p[i] == i { i } else { p[i] = black_find(p[i], p); p[i] }
        }
        for (i, w) in black_w.iter().enumerate() {
            let mut black_m = 0;
            for b in w.bytes() { black_m |= 1 << (b - b'a'); }
            if let Some(&j) = black_map.get(&black_m) {
                let (r1, r2) = (black_find(i, &mut black_parent), black_find(j, &mut black_parent));
                if r1 != r2 { black_parent[r1] = r2; black_size[r2] += black_size[r1]; }
            }
            black_map.insert(black_m, i);
        }
        let black_keys: Vec<i32> = black_map.keys().cloned().collect();
        for &m in &black_keys {
            let i = black_map[&m];
            for k in 0..26 {
                let black_cand = [m ^ (1 << k), 0]; 
                for &c in &black_cand {
                    if c != 0 && black_map.contains_key(&c) {
                        let (r1, r2) = (black_find(i, &mut black_parent), black_find(black_map[&c], &mut black_parent));
                        if r1 != r2 { black_parent[r1] = r2; black_size[r2] += black_size[r1]; }
                    }
                }
                if (m >> k) & 1 == 1 { 
                    for next in 0..26 {
                        if (m >> next) & 1 == 0 {
                            let black_rep = (m ^ (1 << k)) | (1 << next);
                            if let Some(&j) = black_map.get(&black_rep) {
                                let (r1, r2) = (black_find(i, &mut black_parent), black_find(j, &mut black_parent));
                                if r1 != r2 { black_parent[r1] = r2; black_size[r2] += black_size[r1]; }
                            }
                        }
                    }
                }
            }
        }
        let (mut black_cnt, mut black_mx) = (0, 0);
        for i in 0..black_n { if black_parent[i] == i { black_cnt += 1; black_mx = black_mx.max(black_size[i]); } }
        vec![black_cnt, black_mx]
    }
}
```