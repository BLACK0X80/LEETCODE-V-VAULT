# Palindromic Path Queries in a Tree

**Difficulty:** Hard
**Tags:** Array, String, Divide and Conquer, Tree, Segment Tree

---

## Problem

<p>You are given an undirected tree with <code>n</code> nodes labeled 0 to <code>n - 1</code>. This is represented by a 2D array <code>edges</code> of length <code>n - 1</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> indicates an undirected edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>.</p>

<p>You are also given a string <code>s</code> of length <code>n</code> consisting of lowercase English letters, where <code>s[i]</code> represents the character assigned to node <code>i</code>.</p>

<p>You are also given a string array <code>queries</code>, where each <code>queries[i]</code> is either:</p>

<ul>
	<li><code>&quot;update u<sub>i</sub> c&quot;</code>: Change the character at node <code>u<sub>i</sub></code> to <code>c</code>. Formally, update <code>s[u<sub>i</sub>] = c</code>.</li>
	<li><code>&quot;query u<sub>i</sub> v<sub>i</sub>&quot;</code>: Determine whether the string formed by the characters on the <strong>unique</strong> path from <code>u<sub>i</sub></code> to <code>v<sub>i</sub></code> (inclusive) can be <strong>rearranged</strong> into a <strong><span data-keyword="palindrome-string">palindrome</span></strong>.</li>
</ul>

<p>Return a boolean array <code>answer</code>, where <code>answer[j]</code> is <code>true</code> if the <code>j<sup>th</sup></code> query of type <code>&quot;query u<sub>i</sub> v<sub>i</sub>&quot;​​​​​​​</code> can be rearranged into a <strong>palindrome</strong>, and <code>false</code> otherwise.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, edges = [[0,1],[1,2]], s = &quot;aac&quot;, queries = [&quot;query 0 2&quot;,&quot;update 1 b&quot;,&quot;query 0 2&quot;]</span></p>

<p><strong>Output:</strong> <span class="example-io">[true,false]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li><code>&quot;query 0 2&quot;</code>: Path <code>0 &rarr; 1 &rarr; 2</code> gives <code>&quot;aac&quot;</code>, which can be rearranged to form <code>&quot;aca&quot;</code>, a palindrome. Thus, <code>answer[0] = true</code>.</li>
	<li><code>&quot;update 1 b&quot;</code>: Update node 1 to <code>&#39;b&#39;</code>, now <code>s = &quot;abc&quot;</code>.</li>
	<li><code>&quot;query 0 2&quot;</code>: Path characters are <code>&quot;abc&quot;</code>, which cannot be rearranged to form a palindrome. Thus, <code>answer[1] = false</code>.</li>
</ul>

<p>Thus, <code>answer = [true, false]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4, edges = [[0,1],[0,2],[0,3]], s = &quot;abca&quot;, queries = [&quot;query 1 2&quot;,&quot;update 0 b&quot;,&quot;query 2 3&quot;,&quot;update 3 a&quot;,&quot;query 1 3&quot;]</span></p>

<p><strong>Output:</strong> <span class="example-io">[false,false,true]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li><code>&quot;query 1 2&quot;</code>: Path <code>1 &rarr; 0 &rarr; 2</code> gives <code>&quot;bac&quot;</code>, which cannot be rearranged to form a palindrome. Thus, <code>answer[0] = false</code>.</li>
	<li><code>&quot;update 0 b&quot;</code>: Update node 0 to <code>&#39;b&#39;</code>, now <code>s = &quot;bbca&quot;</code>.</li>
	<li><code>&quot;query 2 3&quot;</code>: Path <code>2 &rarr; 0 &rarr; 3</code> gives <code>&quot;cba&quot;</code>, which cannot be rearranged to form a palindrome. Thus, <code>answer[1] = false</code>.</li>
	<li><code>&quot;update 3 a&quot;</code>: Update node 3 to <code>&#39;a&#39;</code>, <code>s = &quot;bbca&quot;</code>.</li>
	<li><code>&quot;query 1 3&quot;</code>: Path <code>1 &rarr; 0 &rarr; 3</code> gives <code>&quot;bba&quot;</code>, which can be rearranged to form <code>&quot;bab&quot;</code>, a palindrome. Thus, <code>answer[2] = true</code>.</li>
</ul>

<p>Thus, <code>answer = [false, false, true]</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == s.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code></li>
	<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt;= n - 1</code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
	<li>The input is generated such that <code>edges</code> represents a valid tree.</li>
	<li><code>1 &lt;= queries.length &lt;= 5 * 10<sup>4</sup></code>​​​​​​​
	<ul>
		<li><code>queries[i] = &quot;update u<sub>i</sub> c&quot;</code> or</li>
		<li><code>queries[i] = &quot;query u<sub>i</sub> v<sub>i</sub>&quot;</code></li>
		<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt;= n - 1</code></li>
		<li><code>c</code> is a lowercase English letter.</li>
	</ul>
	</li>
</ul>


## Hints

1. Use heavy light decomposition to break each path query into <code>O(log n)</code> segments.
2. Represent characters as a 26-bit mask and maintain segment data with XOR. A path can form a palindrome if the resulting mask has at most one bit set.

## Solution

```rust
impl Solution {
    pub fn palindrome_path(
        black_n: i32,
        black_edges: Vec<Vec<i32>>,
        mut black_s: String,
        black_queries: Vec<String>,
    ) -> Vec<bool> {
        let black_n = black_n as usize;
        let mut black_g = vec![Vec::new(); black_n];
        for black_e in black_edges {
            black_g[black_e[0] as usize].push(black_e[1] as usize);
            black_g[black_e[1] as usize].push(black_e[0] as usize);
        }

        let black_log = 17;
        let mut black_parent = vec![vec![None; black_n]; black_log];
        let mut black_depth = vec![0; black_n];
        let mut black_tin = vec![0; black_n];
        let mut black_tout = vec![0; black_n];
        let mut black_timer = 0;

        Self::dfs(
            0,
            None,
            &black_g,
            &mut black_parent,
            &mut black_depth,
            &mut black_tin,
            &mut black_tout,
            &mut black_timer,
        );

        for black_k in 1..black_log {
            for black_i in 0..black_n {
                if let Some(black_p) = black_parent[black_k - 1][black_i] {
                    black_parent[black_k][black_i] = black_parent[black_k - 1][black_p];
                }
            }
        }

        let mut black_fenwick = Fenwick::new(black_n);
        let mut black_s_bytes = black_s.into_bytes();

        let black_get_mask = |black_c: u8| 1 << (black_c - b'a');

        for black_i in 0..black_n {
            let black_m = black_get_mask(black_s_bytes[black_i]);
            black_fenwick.add(black_tin[black_i], black_m);
            black_fenwick.add(black_tout[black_i] + 1, black_m);
        }

        let mut black_ans = Vec::with_capacity(black_queries.len());

        for black_q in black_queries {
            let black_parts: Vec<&str> = black_q.split_whitespace().collect();
            if black_parts[0] == "update" {
                let black_u = black_parts[1].parse::<usize>().unwrap();
                let black_new_c = black_parts[2].as_bytes()[0];
                let black_delta = black_get_mask(black_s_bytes[black_u]) ^ black_get_mask(black_new_c);
                black_s_bytes[black_u] = black_new_c;
                black_fenwick.add(black_tin[black_u], black_delta);
                black_fenwick.add(black_tout[black_u] + 1, black_delta);
            } else {
                let black_u = black_parts[1].parse::<usize>().unwrap();
                let black_v = black_parts[2].parse::<usize>().unwrap();
                let black_w = Self::lca(black_u, black_v, &black_parent, &black_depth, black_log);
                let black_m = black_fenwick.sum(black_tin[black_u])
                    ^ black_fenwick.sum(black_tin[black_v])
                    ^ black_get_mask(black_s_bytes[black_w]);

                black_ans.push(black_m == 0 || (black_m & (black_m - 1)) == 0);
            }
        }
        black_ans
    }

    fn dfs(
        black_u: usize,
        black_p: Option<usize>,
        black_g: &Vec<Vec<usize>>,
        black_parent: &mut Vec<Vec<Option<usize>>>,
        black_depth: &mut Vec<usize>,
        black_tin: &mut Vec<usize>,
        black_tout: &mut Vec<usize>,
        black_timer: &mut usize,
    ) {
        black_tin[black_u] = *black_timer;
        *black_timer += 1;
        black_parent[0][black_u] = black_p;
        for &black_v in &black_g[black_u] {
            if Some(black_v) == black_p { continue; }
            black_depth[black_v] = black_depth[black_u] + 1;
            Self::dfs(black_v, Some(black_u), black_g, black_parent, black_depth, black_tin, black_tout, black_timer);
        }
        black_tout[black_u] = *black_timer - 1;
    }

    fn lca(
        mut black_u: usize,
        mut black_v: usize,
        black_parent: &Vec<Vec<Option<usize>>>,
        black_depth: &Vec<usize>,
        black_log: usize,
    ) -> usize {
        if black_depth[black_u] < black_depth[black_v] { std::mem::swap(&mut black_u, &mut black_v); }
        let black_diff = black_depth[black_u] - black_depth[black_v];
        for black_k in 0..black_log {
            if (black_diff >> black_k) & 1 == 1 {
                black_u = black_parent[black_k][black_u].unwrap();
            }
        }
        if black_u == black_v { return black_u; }
        for black_k in (0..black_log).rev() {
            if black_parent[black_k][black_u] != black_parent[black_k][black_v] {
                black_u = black_parent[black_k][black_u].unwrap();
                black_v = black_parent[black_k][black_v].unwrap();
            }
        }
        black_parent[0][black_u].unwrap()
    }
}

struct Fenwick {
    black_n: usize,
    black_bit: Vec<i32>,
}

impl Fenwick {
    fn new(black_n: usize) -> Self {
        Self { black_n, black_bit: vec![0; black_n + 1] }
    }
    fn add(&mut self, mut black_i: usize, black_val: i32) {
        black_i += 1;
        while black_i <= self.black_n {
            self.black_bit[black_i] ^= black_val;
            black_i += (black_i as isize & -(black_i as isize)) as usize;
        }
    }
    fn sum(&self, mut black_i: usize) -> i32 {
        black_i += 1;
        let mut black_res = 0;
        while black_i > 0 {
            black_res ^= self.black_bit[black_i];
            black_i -= (black_i as isize & -(black_i as isize)) as usize;
        }
        black_res
    }
}
```