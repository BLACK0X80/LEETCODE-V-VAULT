# Word Search II

**Difficulty:** Hard
**Tags:** Array, String, Backtracking, Trie, Matrix

---

## Problem

<p>Given an <code>m x n</code> <code>board</code>&nbsp;of characters and a list of strings <code>words</code>, return <em>all words on the board</em>.</p>

<p>Each word must be constructed from letters of sequentially adjacent cells, where <strong>adjacent cells</strong> are horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/07/search1.jpg" style="width: 322px; height: 322px;" />
<pre>
<strong>Input:</strong> board = [[&quot;o&quot;,&quot;a&quot;,&quot;a&quot;,&quot;n&quot;],[&quot;e&quot;,&quot;t&quot;,&quot;a&quot;,&quot;e&quot;],[&quot;i&quot;,&quot;h&quot;,&quot;k&quot;,&quot;r&quot;],[&quot;i&quot;,&quot;f&quot;,&quot;l&quot;,&quot;v&quot;]], words = [&quot;oath&quot;,&quot;pea&quot;,&quot;eat&quot;,&quot;rain&quot;]
<strong>Output:</strong> [&quot;eat&quot;,&quot;oath&quot;]
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/07/search2.jpg" style="width: 162px; height: 162px;" />
<pre>
<strong>Input:</strong> board = [[&quot;a&quot;,&quot;b&quot;],[&quot;c&quot;,&quot;d&quot;]], words = [&quot;abcb&quot;]
<strong>Output:</strong> []
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == board.length</code></li>
	<li><code>n == board[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 12</code></li>
	<li><code>board[i][j]</code> is a lowercase English letter.</li>
	<li><code>1 &lt;= words.length &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= words[i].length &lt;= 10</code></li>
	<li><code>words[i]</code> consists of lowercase English letters.</li>
	<li>All the strings of <code>words</code> are unique.</li>
</ul>


## Hints

1. You would need to optimize your backtracking to pass the larger test. Could you stop backtracking earlier?
2. If the current candidate does not exist in all words&#39; prefix, you could stop backtracking immediately. What kind of data structure could answer such query efficiently? Does a hash table work? Why or why not? How about a Trie? If you would like to learn how to implement a basic trie, please work on this problem: <a href="https://leetcode.com/problems/implement-trie-prefix-tree/">Implement Trie (Prefix Tree)</a> first.

## Solution

```rust
impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = vec![[0usize; 26]; 1];
        let mut ends = vec![0usize];
        let mut word_id = 1;

        for word in &words {
            let mut node = 0;
            for c in word.bytes().map(|b| (b - b'a') as usize) {
                if trie[node][c] == 0 {
                    trie.push([0; 26]);
                    ends.push(0);
                    trie[node][c] = trie.len() - 1;
                }
                node = trie[node][c];
            }
            ends[node] = word_id;
            word_id += 1;
        }

        let mut result = vec![];
        let (m, n) = (board.len(), board[0].len());

        for i in 0..m {
            for j in 0..n {
                Self::dfs(&mut board, i, j, 0, &mut trie, &mut ends, &words, &mut result);
            }
        }

        result
    }

    fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize, node: usize,
           trie: &mut Vec<[usize; 26]>, ends: &mut Vec<usize>,
           words: &[String], result: &mut Vec<String>) {
        let c = board[i][j];
        if c == '#' { return; }
        let ci = (c as u8 - b'a') as usize;
        let next = trie[node][ci];
        if next == 0 { return; }

        if ends[next] > 0 {
            result.push(words[ends[next] - 1].clone());
            ends[next] = 0;
        }

        board[i][j] = '#';
        let dirs = [(0,1),(0,-1),(1,0),(-1,0)];
        for (di, dj) in dirs {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && nj >= 0 && (ni as usize) < board.len() && (nj as usize) < board[0].len() {
                Self::dfs(board, ni as usize, nj as usize, next, trie, ends, words, result);
            }
        }
        board[i][j] = c;
    }
}
```