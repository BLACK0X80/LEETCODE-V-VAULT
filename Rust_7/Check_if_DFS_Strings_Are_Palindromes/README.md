# Check if DFS Strings Are Palindromes

**Difficulty:** Hard
**Tags:** Array, Hash Table, String, Tree, Depth-First Search, Hash Function

---

## Problem

<p>You are given a tree rooted at node 0, consisting of <code>n</code> nodes numbered from <code>0</code> to <code>n - 1</code>. The tree is represented by an array <code>parent</code> of size <code>n</code>, where <code>parent[i]</code> is the parent of node <code>i</code>. Since node 0 is the root, <code>parent[0] == -1</code>.</p>

<p>You are also given a string <code>s</code> of length <code>n</code>, where <code>s[i]</code> is the character assigned to node <code>i</code>.</p>

<p>Consider an empty string <code>dfsStr</code>, and define a recursive function <code>dfs(int x)</code> that takes a node <code>x</code> as a parameter and performs the following steps in order:</p>

<ul>
	<li>Iterate over each child <code>y</code> of <code>x</code> <strong>in increasing order of their numbers</strong>, and call <code>dfs(y)</code>.</li>
	<li>Add the character <code>s[x]</code> to the end of the string <code>dfsStr</code>.</li>
</ul>

<p><strong>Note</strong> that <code>dfsStr</code> is shared across all recursive calls of <code>dfs</code>.</p>

<p>You need to find a boolean array <code>answer</code> of size <code>n</code>, where for each index <code>i</code> from <code>0</code> to <code>n - 1</code>, you do the following:</p>

<ul>
	<li>Empty the string <code>dfsStr</code> and call <code>dfs(i)</code>.</li>
	<li>If the resulting string <code>dfsStr</code> is a <span data-keyword="palindrome-string">palindrome</span>, then set <code>answer[i]</code> to <code>true</code>. Otherwise, set <code>answer[i]</code> to <code>false</code>.</li>
</ul>

<p>Return the array <code>answer</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2024/09/01/tree1drawio.png" style="width: 240px; height: 256px;" />
<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">parent = [-1,0,0,1,1,2], s = &quot;aababa&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">[true,true,false,true,true,true]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Calling <code>dfs(0)</code> results in the string <code>dfsStr = &quot;abaaba&quot;</code>, which is a palindrome.</li>
	<li>Calling <code>dfs(1)</code> results in the string <code>dfsStr = &quot;aba&quot;</code>, which is a palindrome.</li>
	<li>Calling <code>dfs(2)</code> results in the string <code>dfsStr = &quot;ab&quot;</code>, which is <strong>not</strong> a palindrome.</li>
	<li>Calling <code>dfs(3)</code> results in the string <code>dfsStr = &quot;a&quot;</code>, which is a palindrome.</li>
	<li>Calling <code>dfs(4)</code> results in the string <code>dfsStr = &quot;b&quot;</code>, which is a palindrome.</li>
	<li>Calling <code>dfs(5)</code> results in the string <code>dfsStr = &quot;a&quot;</code>, which is a palindrome.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2024/09/01/tree2drawio-1.png" style="width: 260px; height: 167px;" />
<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">parent = [-1,0,0,0,0], s = &quot;aabcb&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">[true,true,true,true,true]</span></p>

<p><strong>Explanation:</strong></p>

<p>Every call on <code>dfs(x)</code> results in a palindrome string.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == parent.length == s.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= parent[i] &lt;= n - 1</code> for all <code>i &gt;= 1</code>.</li>
	<li><code>parent[0] == -1</code></li>
	<li><code>parent</code> represents a valid tree.</li>
	<li><code>s</code> consists only of lowercase English letters.</li>
</ul>


## Hints

1. Perform the dfs described from the root of tree, and store the order in which nodes are visited into an array.
2. For any node in the tree, the nodes in its subtree will form a contiguous subarray within the DFS traversal array.
3. Use Manacher’s algorithm to compute the answer for each node in constant time.

## Solution

```rust
impl Solution {
    pub fn find_answer(black_parent: Vec<i32>, black_s: String) -> Vec<bool> {
        let black_n = black_parent.len();
        let black_s_bytes = black_s.as_bytes();
        let mut black_adj = vec![vec![]; black_n];
        for (black_i, &black_p) in black_parent.iter().enumerate() {
            if black_p != -1 {
                black_adj[black_p as usize].push(black_i);
            }
        }
        for black_i in 0..black_n {
            black_adj[black_i].sort();
        }

        let mut black_dfs_order = Vec::with_capacity(black_n);
        let mut black_entry = vec![0; black_n];
        let mut black_exit = vec![0; black_n];
        
        Self::black_build_order(0, &black_adj, black_s_bytes, &mut black_dfs_order, &mut black_entry, &mut black_exit);

        let black_m = black_dfs_order.len();
        let black_base: u64 = 31;
        let black_mod: u64 = 1_000_000_000 + 7;

        let mut black_pow = vec![1; black_m + 1];
        let mut black_h1 = vec![0; black_m + 1];
        let mut black_h2 = vec![0; black_m + 1];

        for black_i in 0..black_m {
            black_pow[black_i + 1] = (black_pow[black_i] * black_base) % black_mod;
            black_h1[black_i + 1] = (black_h1[black_i] * black_base + (black_dfs_order[black_i] - b'a' + 1) as u64) % black_mod;
            black_h2[black_i + 1] = (black_h2[black_i] * black_base + (black_dfs_order[black_m - 1 - black_i] - b'a' + 1) as u64) % black_mod;
        }

        let mut black_res = vec![false; black_n];
        for black_i in 0..black_n {
            let black_l = black_entry[black_i];
            let black_r = black_exit[black_i];
            let black_len = black_r - black_l + 1;

            let black_f_hash = (black_h1[black_r + 1] + black_mod - (black_h1[black_l] * black_pow[black_len]) % black_mod) % black_mod;
            
            let black_rev_l = black_m - 1 - black_r;
            let black_rev_r = black_m - 1 - black_l;
            let black_b_hash = (black_h2[black_rev_r + 1] + black_mod - (black_h2[black_rev_l] * black_pow[black_len]) % black_mod) % black_mod;

            if black_f_hash == black_b_hash {
                black_res[black_i] = true;
            }
        }

        black_res
    }

    fn black_build_order(
        black_u: usize,
        black_adj: &Vec<Vec<usize>>,
        black_s: &[u8],
        black_order: &mut Vec<u8>,
        black_entry: &mut Vec<usize>,
        black_exit: &mut Vec<usize>,
    ) {
        black_entry[black_u] = black_order.len();
        for &black_v in &black_adj[black_u] {
            Self::black_build_order(black_v, black_adj, black_s, black_order, black_entry, black_exit);
        }
        black_order.push(black_s[black_u]);
        black_exit[black_u] = black_order.len() - 1;
    }
}
```