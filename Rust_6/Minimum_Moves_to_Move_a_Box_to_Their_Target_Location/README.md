# Minimum Moves to Move a Box to Their Target Location

**Difficulty:** Hard
**Tags:** Array, Breadth-First Search, Heap (Priority Queue), Matrix

---

## Problem

<p>A storekeeper is a game in which the player pushes boxes around in a warehouse trying to get them to target locations.</p>

<p>The game is represented by an <code>m x n</code> grid of characters <code>grid</code> where each element is a wall, floor, or box.</p>

<p>Your task is to move the box <code>&#39;B&#39;</code> to the target position <code>&#39;T&#39;</code> under the following rules:</p>

<ul>
	<li>The character <code>&#39;S&#39;</code> represents the player. The player can move up, down, left, right in <code>grid</code> if it is a floor (empty cell).</li>
	<li>The character <code>&#39;.&#39;</code> represents the floor which means a free cell to walk.</li>
	<li>The character<font face="monospace">&nbsp;</font><code>&#39;#&#39;</code><font face="monospace">&nbsp;</font>represents the wall which means an obstacle (impossible to walk there).</li>
	<li>There is only one box <code>&#39;B&#39;</code> and one target cell <code>&#39;T&#39;</code> in the <code>grid</code>.</li>
	<li>The box can be moved to an adjacent free cell by standing next to the box and then moving in the direction of the box. This is a <strong>push</strong>.</li>
	<li>The player cannot walk through the box.</li>
</ul>

<p>Return <em>the minimum number of <strong>pushes</strong> to move the box to the target</em>. If there is no way to reach the target, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/11/06/sample_1_1620.png" style="width: 500px; height: 335px;" />
<pre>
<strong>Input:</strong> grid = [[&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;T&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;.&quot;,&quot;.&quot;,&quot;B&quot;,&quot;.&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;.&quot;,&quot;#&quot;,&quot;#&quot;,&quot;.&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;.&quot;,&quot;.&quot;,&quot;.&quot;,&quot;S&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> We return only the number of times the box is pushed.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> grid = [[&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;T&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;.&quot;,&quot;.&quot;,&quot;B&quot;,&quot;.&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;.&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;.&quot;,&quot;.&quot;,&quot;.&quot;,&quot;S&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;]]
<strong>Output:</strong> -1
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> grid = [[&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;T&quot;,&quot;.&quot;,&quot;.&quot;,&quot;#&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;.&quot;,&quot;#&quot;,&quot;B&quot;,&quot;.&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;.&quot;,&quot;.&quot;,&quot;.&quot;,&quot;.&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;.&quot;,&quot;.&quot;,&quot;.&quot;,&quot;S&quot;,&quot;#&quot;],
               [&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;,&quot;#&quot;]]
<strong>Output:</strong> 5
<strong>Explanation:</strong> push the box down, left, left, up and up.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 20</code></li>
	<li><code>grid</code> contains only characters <code>&#39;.&#39;</code>, <code>&#39;#&#39;</code>, <code>&#39;S&#39;</code>, <code>&#39;T&#39;</code>, or <code>&#39;B&#39;</code>.</li>
	<li>There is only one character <code>&#39;S&#39;</code>, <code>&#39;B&#39;</code>, and <code>&#39;T&#39;</code> in the <code>grid</code>.</li>
</ul>


## Hints

1. We represent the search state as (player_row, player_col, box_row, box_col).
2. You need to count only the number of pushes. Then inside of your BFS check if the box could be pushed (in any direction) given the current position of the player.

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut sr,mut sc,mut br,mut bc,mut tr,mut tc) = (0,0,0,0,0,0);
        for i in 0..m { for j in 0..n { match grid[i][j] { 'S'=>{sr=i;sc=j;} 'B'=>{br=i;bc=j;} 'T'=>{tr=i;tc=j;} _=>{} } } }

        let can_reach = |pr:usize,pc:usize,er:usize,ec:usize,bor:usize,boc:usize| -> bool {
            let mut vis = vec![vec![false;n];m];
            let mut q = VecDeque::new();
            q.push_back((pr,pc)); vis[pr][pc]=true;
            while let Some((r,c)) = q.pop_front() {
                if r==er && c==ec { return true; }
                for (dr,dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                    let nr=r as i32+dr; let nc=c as i32+dc;
                    if nr<0||nc<0||nr>=m as i32||nc>=n as i32 { continue; }
                    let (nr,nc)=(nr as usize,nc as usize);
                    if !vis[nr][nc] && grid[nr][nc]!='#' && (nr,nc)!=(bor,boc) { vis[nr][nc]=true; q.push_back((nr,nc)); }
                }
            }
            false
        };

        let mut vis = vec![vec![vec![vec![false;n];m];n];m];
        let mut q = VecDeque::new();
        q.push_back((br,bc,sr,sc,0));
        vis[br][bc][sr][sc]=true;
        while let Some((br,bc,pr,pc,pushes)) = q.pop_front() {
            if br==tr && bc==tc { return pushes; }
            for (dr,dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                let nbr=br as i32+dr; let nbc=bc as i32+dc;
                if nbr<0||nbc<0||nbr>=m as i32||nbc>=n as i32 { continue; }
                let (nbr,nbc)=(nbr as usize,nbc as usize);
                if grid[nbr][nbc]=='#' { continue; }
                let need_r=(br as i32-dr) as usize; let need_c=(bc as i32-dc) as usize;
                if can_reach(pr,pc,need_r,need_c,br,bc) && !vis[nbr][nbc][br][bc] {
                    vis[nbr][nbc][br][bc]=true;
                    q.push_back((nbr,nbc,br,bc,pushes+1));
                }
            }
        }
        -1
    }
}
```