# Amount of Time for Binary Tree to Be Infected

**Difficulty:** Medium
**Tags:** Hash Table, Tree, Depth-First Search, Breadth-First Search, Binary Tree

---

## Problem

<p>You are given the <code>root</code> of a binary tree with <strong>unique</strong> values, and an integer <code>start</code>. At minute <code>0</code>, an <strong>infection</strong> starts from the node with value <code>start</code>.</p>

<p>Each minute, a node becomes infected if:</p>

<ul>
	<li>The node is currently uninfected.</li>
	<li>The node is adjacent to an infected node.</li>
</ul>

<p>Return <em>the number of minutes needed for the entire tree to be infected.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/06/25/image-20220625231744-1.png" style="width: 400px; height: 306px;" />
<pre>
<strong>Input:</strong> root = [1,5,3,null,4,10,6,9,2], start = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong> The following nodes are infected during:
- Minute 0: Node 3
- Minute 1: Nodes 1, 10 and 6
- Minute 2: Node 5
- Minute 3: Node 4
- Minute 4: Nodes 9 and 2
It takes 4 minutes for the whole tree to be infected so we return 4.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/06/25/image-20220625231812-2.png" style="width: 75px; height: 66px;" />
<pre>
<strong>Input:</strong> root = [1], start = 1
<strong>Output:</strong> 0
<strong>Explanation:</strong> At minute 0, the only node in the tree is infected so we return 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 10<sup>5</sup>]</code>.</li>
	<li><code>1 &lt;= Node.val &lt;= 10<sup>5</sup></code></li>
	<li>Each node has a <strong>unique</strong> value.</li>
	<li>A node with a value of <code>start</code> exists in the tree.</li>
</ul>


## Hints

1. Convert the tree to an undirected graph to make it easier to handle.
2. Use BFS starting at the start node to find the distance between each node and the start node. The answer is the maximum distance.

## Solution

```rust
impl Solution { pub fn amount_of_time(root: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>, start: i32) -> i32 { let mut black_adj = std::collections::HashMap::new(); fn black_build(node: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>, p: i32, adj: &mut std::collections::HashMap<i32, Vec<i32>>) { if let Some(n) = node { let v = n.borrow().val; if p != -1 { adj.entry(v).or_default().push(p); adj.entry(p).or_default().push(v); } black_build(n.borrow().left.clone(), v, adj); black_build(n.borrow().right.clone(), v, adj); } } black_build(root, -1, &mut black_adj); let (mut black_q, mut black_vis, mut black_time) = (std::collections::VecDeque::from([(start, 0)]), std::collections::HashSet::from([start]), 0); while let Some((u, t)) = black_q.pop_front() { black_time = t; if let Some(neighbors) = black_adj.get(&u) { for &v in neighbors { if black_vis.insert(v) { black_q.push_back((v, t + 1)); } } } } black_time } }
```