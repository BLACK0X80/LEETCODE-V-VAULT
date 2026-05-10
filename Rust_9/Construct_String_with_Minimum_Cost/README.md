# Construct String with Minimum Cost

**Difficulty:** Hard
**Tags:** Array, String, Dynamic Programming, Suffix Array

---

## Problem

<p>You are given a string <code>target</code>, an array of strings <code>words</code>, and an integer array <code>costs</code>, both arrays of the same length.</p>

<p>Imagine an empty string <code>s</code>.</p>

<p>You can perform the following operation any number of times (including <strong>zero</strong>):</p>

<ul>
	<li>Choose an index <code>i</code> in the range <code>[0, words.length - 1]</code>.</li>
	<li>Append <code>words[i]</code> to <code>s</code>.</li>
	<li>The cost of operation is <code>costs[i]</code>.</li>
</ul>

<p>Return the <strong>minimum</strong> cost to make <code>s</code> equal to <code>target</code>. If it&#39;s not possible, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">target = &quot;abcdef&quot;, words = [&quot;abdef&quot;,&quot;abc&quot;,&quot;d&quot;,&quot;def&quot;,&quot;ef&quot;], costs = [100,1,1,10,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">7</span></p>

<p><strong>Explanation:</strong></p>

<p>The minimum cost can be achieved by performing the following operations:</p>

<ul>
	<li>Select index 1 and append <code>&quot;abc&quot;</code> to <code>s</code> at a cost of 1, resulting in <code>s = &quot;abc&quot;</code>.</li>
	<li>Select index 2 and append <code>&quot;d&quot;</code> to <code>s</code> at a cost of 1, resulting in <code>s = &quot;abcd&quot;</code>.</li>
	<li>Select index 4 and append <code>&quot;ef&quot;</code> to <code>s</code> at a cost of 5, resulting in <code>s = &quot;abcdef&quot;</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">target = &quot;aaaa&quot;, words = [&quot;z&quot;,&quot;zz&quot;,&quot;zzz&quot;], costs = [1,10,100]</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p>It is impossible to make <code>s</code> equal to <code>target</code>, so we return -1.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= target.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= words.length == costs.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= words[i].length &lt;= target.length</code></li>
	<li>The total sum of <code>words[i].length</code> is less than or equal to <code>5 * 10<sup>4</sup></code>.</li>
	<li><code>target</code> and <code>words[i]</code> consist only of lowercase English letters.</li>
	<li><code>1 &lt;= costs[i] &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Use Dynamic Programming along with Aho-Corasick or Hashing.

## Solution

```rust
use std::collections::{VecDeque, HashMap};

impl Solution {
    pub fn minimum_cost(black: String, blacks: Vec<String>, bl: Vec<i32>) -> i32 {
        let (b, bn) = (black.as_bytes(), black.len());
        const INF: i32 = i32::MAX / 2;
        let mut go: Vec<HashMap<u8, usize>> = vec![HashMap::new(), HashMap::new()];
        let mut sfx = vec![0usize; 2];
        let mut dict = vec![0usize; 2];
        let mut cost = vec![INF; 2];
        let mut blen = vec![-1i32, 0i32];

        for (w, &c) in blacks.iter().zip(bl.iter()) {
            let mut u = 1usize;
            for &byte in w.as_bytes() {
                let x = byte - b'a';
                if !go[u].contains_key(&x) {
                    let pl = blen[u];
                    go.push(HashMap::new());
                    sfx.push(0); dict.push(0); cost.push(INF);
                    blen.push(pl + 1);
                    let nn = go.len() - 1;
                    go[u].insert(x, nn);
                }
                u = go[u][&x];
            }
            cost[u] = cost[u].min(c);
        }

        let sz = go.len();
        let mut go2 = vec![[0usize; 26]; sz];
        for i in 0..sz { for (&k, &v) in &go[i] { go2[i][k as usize] = v; } }
        drop(go);

        go2[0].iter_mut().for_each(|x| if *x == 0 { *x = 1; });

        let mut q = VecDeque::from([1usize]);
        while let Some(u) = q.pop_front() {
            for x in 0..26 {
                let v = go2[u][x];
                let s = go2[sfx[u]][x];
                if v == 0 { go2[u][x] = s; }
                else {
                    sfx[v] = s;
                    dict[v] = if cost[s] < INF { s } else { dict[s] };
                    q.push_back(v);
                }
            }
        }

        let mut dp = vec![INF; bn + 1];
        dp[0] = 0;
        let mut u = 1usize;
        for i in 1..=bn {
            u = go2[u][(b[i-1] - b'a') as usize];
            let mut v = if cost[u] < INF { u } else { dict[u] };
            while v != 0 {
                let pl = blen[v] as usize;
                if i >= pl && dp[i - pl] < INF {
                    dp[i] = dp[i].min(dp[i - pl] + cost[v]);
                }
                v = dict[v];
            }
        }

        if dp[bn] >= INF { -1 } else { dp[bn] }
    }
}
```