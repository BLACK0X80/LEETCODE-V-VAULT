# Minimum Cost to Convert String II

**Difficulty:** Hard
**Tags:** Array, String, Dynamic Programming, Graph Theory, Trie, Shortest Path

---

## Problem

<p>You are given two <strong>0-indexed</strong> strings <code>source</code> and <code>target</code>, both of length <code>n</code> and consisting of <strong>lowercase</strong> English characters. You are also given two <strong>0-indexed</strong> string arrays <code>original</code> and <code>changed</code>, and an integer array <code>cost</code>, where <code>cost[i]</code> represents the cost of converting the string <code>original[i]</code> to the string <code>changed[i]</code>.</p>

<p>You start with the string <code>source</code>. In one operation, you can pick a <strong>substring</strong> <code>x</code> from the string, and change it to <code>y</code> at a cost of <code>z</code> <strong>if</strong> there exists <strong>any</strong> index <code>j</code> such that <code>cost[j] == z</code>, <code>original[j] == x</code>, and <code>changed[j] == y</code>. You are allowed to do <strong>any</strong> number of operations, but any pair of operations must satisfy <strong>either</strong> of these two conditions:</p>

<ul>
	<li>The substrings picked in the operations are <code>source[a..b]</code> and <code>source[c..d]</code> with either <code>b &lt; c</code> <strong>or</strong> <code>d &lt; a</code>. In other words, the indices picked in both operations are <strong>disjoint</strong>.</li>
	<li>The substrings picked in the operations are <code>source[a..b]</code> and <code>source[c..d]</code> with <code>a == c</code> <strong>and</strong> <code>b == d</code>. In other words, the indices picked in both operations are <strong>identical</strong>.</li>
</ul>

<p>Return <em>the <strong>minimum</strong> cost to convert the string </em><code>source</code><em> to the string </em><code>target</code><em> using <strong>any</strong> number of operations</em>. <em>If it is impossible to convert</em> <code>source</code> <em>to</em> <code>target</code>,<em> return</em> <code>-1</code>.</p>

<p><strong>Note</strong> that there may exist indices <code>i</code>, <code>j</code> such that <code>original[j] == original[i]</code> and <code>changed[j] == changed[i]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> source = &quot;abcd&quot;, target = &quot;acbe&quot;, original = [&quot;a&quot;,&quot;b&quot;,&quot;c&quot;,&quot;c&quot;,&quot;e&quot;,&quot;d&quot;], changed = [&quot;b&quot;,&quot;c&quot;,&quot;b&quot;,&quot;e&quot;,&quot;b&quot;,&quot;e&quot;], cost = [2,5,5,1,2,20]
<strong>Output:</strong> 28
<strong>Explanation:</strong> To convert &quot;abcd&quot; to &quot;acbe&quot;, do the following operations:
- Change substring source[1..1] from &quot;b&quot; to &quot;c&quot; at a cost of 5.
- Change substring source[2..2] from &quot;c&quot; to &quot;e&quot; at a cost of 1.
- Change substring source[2..2] from &quot;e&quot; to &quot;b&quot; at a cost of 2.
- Change substring source[3..3] from &quot;d&quot; to &quot;e&quot; at a cost of 20.
The total cost incurred is 5 + 1 + 2 + 20 = 28. 
It can be shown that this is the minimum possible cost.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> source = &quot;abcdefgh&quot;, target = &quot;acdeeghh&quot;, original = [&quot;bcd&quot;,&quot;fgh&quot;,&quot;thh&quot;], changed = [&quot;cde&quot;,&quot;thh&quot;,&quot;ghh&quot;], cost = [1,3,5]
<strong>Output:</strong> 9
<strong>Explanation:</strong> To convert &quot;abcdefgh&quot; to &quot;acdeeghh&quot;, do the following operations:
- Change substring source[1..3] from &quot;bcd&quot; to &quot;cde&quot; at a cost of 1.
- Change substring source[5..7] from &quot;fgh&quot; to &quot;thh&quot; at a cost of 3. We can do this operation because indices [5,7] are disjoint with indices picked in the first operation.
- Change substring source[5..7] from &quot;thh&quot; to &quot;ghh&quot; at a cost of 5. We can do this operation because indices [5,7] are disjoint with indices picked in the first operation, and identical with indices picked in the second operation.
The total cost incurred is 1 + 3 + 5 = 9.
It can be shown that this is the minimum possible cost.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> source = &quot;abcdefgh&quot;, target = &quot;addddddd&quot;, original = [&quot;bcd&quot;,&quot;defgh&quot;], changed = [&quot;ddd&quot;,&quot;ddddd&quot;], cost = [100,1578]
<strong>Output:</strong> -1
<strong>Explanation:</strong> It is impossible to convert &quot;abcdefgh&quot; to &quot;addddddd&quot;.
If you select substring source[1..3] as the first operation to change &quot;abcdefgh&quot; to &quot;adddefgh&quot;, you cannot select substring source[3..7] as the second operation because it has a common index, 3, with the first operation.
If you select substring source[3..7] as the first operation to change &quot;abcdefgh&quot; to &quot;abcddddd&quot;, you cannot select substring source[1..3] as the second operation because it has a common index, 3, with the first operation.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= source.length == target.length &lt;= 1000</code></li>
	<li><code>source</code>, <code>target</code> consist only of lowercase English characters.</li>
	<li><code>1 &lt;= cost.length == original.length == changed.length &lt;= 100</code></li>
	<li><code>1 &lt;= original[i].length == changed[i].length &lt;= source.length</code></li>
	<li><code>original[i]</code>, <code>changed[i]</code> consist only of lowercase English characters.</li>
	<li><code>original[i] != changed[i]</code></li>
	<li><code>1 &lt;= cost[i] &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Give each unique string in <code>original</code> and <code>changed</code> arrays a unique id. There are at most <code>2 * m</code> unique strings in total where <code>m</code> is the length of the arrays. We can put them into a hash map to assign ids.
2. We can pre-compute the smallest costs between all pairs of unique strings using Floyd Warshall algorithm in <code>O(m ^ 3)</code> time complexity.
3. Let <code>dp[i]</code> be the smallest cost to change the first <code>i</code> characters (prefix) of <code>source</code> into <code>target</code>, leaving the suffix untouched.
We have <code>dp[0] = 0</code>.
<code>dp[i] = min(
dp[i - 1] if (source[i - 1] == target[i - 1]),
dp[j-1] + cost[x][y] where x is the id of source[j..(i - 1)] and y is the id of target e[j..(i - 1)])
)</code>.
If neither of the two conditions is satisfied, <code>dp[i] = infinity</code>.
4. We can use Trie to check for the second condition in <code>O(1)</code>.
5. The answer is <code>dp[n]</code> where <code>n</code> is <code>source.length</code>.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn minimum_cost(source: String, target: String, original: Vec<String>, changed: Vec<String>, cost: Vec<i32>) -> i64 {
        let src = source.as_bytes();
        let tgt = target.as_bytes();
        let n = src.len();
        
        let mut by_len: HashMap<usize, HashMap<(String, String), i64>> = HashMap::new();
        for ((o, c), &w) in original.iter().zip(changed.iter()).zip(cost.iter()) {
            let len = o.len();
            let key = (o.clone(), c.clone());
            by_len.entry(len).or_default().entry(key).and_modify(|e| *e = (*e).min(w as i64)).or_insert(w as i64);
        }
        
        let mut all_costs: HashMap<usize, HashMap<(String, String), i64>> = HashMap::new();
        
        for (len, graph) in by_len {
            let mut nodes: Vec<String> = Vec::new();
            let mut node_idx: HashMap<String, usize> = HashMap::new();
            
            for (o, c) in graph.keys() {
                if !node_idx.contains_key(o) {
                    node_idx.insert(o.clone(), nodes.len());
                    nodes.push(o.clone());
                }
                if !node_idx.contains_key(c) {
                    node_idx.insert(c.clone(), nodes.len());
                    nodes.push(c.clone());
                }
            }
            
            let sz = nodes.len();
            let mut dist = vec![vec![i64::MAX / 4; sz]; sz];
            for i in 0..sz { dist[i][i] = 0; }
            
            for ((o, c), &w) in &graph {
                let u = node_idx[o];
                let v = node_idx[c];
                dist[u][v] = dist[u][v].min(w);
            }
            
            for k in 0..sz {
                for i in 0..sz {
                    if dist[i][k] == i64::MAX / 4 { continue; }
                    for j in 0..sz {
                        if dist[k][j] == i64::MAX / 4 { continue; }
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
            
            let mut result: HashMap<(String, String), i64> = HashMap::new();
            for i in 0..sz {
                for j in 0..sz {
                    if dist[i][j] < i64::MAX / 4 {
                        result.insert((nodes[i].clone(), nodes[j].clone()), dist[i][j]);
                    }
                }
            }
            all_costs.insert(len, result);
        }
        
        let mut dp = vec![i64::MAX / 4; n + 1];
        dp[0] = 0;
        
        for i in 0..n {
            if dp[i] == i64::MAX / 4 { continue; }
            
            if src[i] == tgt[i] {
                dp[i + 1] = dp[i + 1].min(dp[i]);
            }
            
            for (&len, costs) in &all_costs {
                if i + len > n { continue; }
                
                let s = std::str::from_utf8(&src[i..i+len]).unwrap();
                let t = std::str::from_utf8(&tgt[i..i+len]).unwrap();
                
                if let Some(&c) = costs.get(&(s.to_string(), t.to_string())) {
                    dp[i + len] = dp[i + len].min(dp[i] + c);
                }
            }
        }
        
        if dp[n] == i64::MAX / 4 { -1 } else { dp[n] }
    }
}
```