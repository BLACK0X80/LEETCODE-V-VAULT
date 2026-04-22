# N-ary Tree Level Order Traversal

**Difficulty:** Medium
**Tags:** Tree, Breadth-First Search

---

## Problem

<p>Given an n-ary tree, return the <em>level order</em> traversal of its nodes&#39; values.</p>

<p><em>Nary-Tree input serialization is represented in their level order traversal, each group of children is separated by the null value (See examples).</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2018/10/12/narytreeexample.png" style="width: 100%; max-width: 300px;" /></p>

<pre>
<strong>Input:</strong> root = [1,null,3,2,4,null,5,6]
<strong>Output:</strong> [[1],[3,2,4],[5,6]]
</pre>

<p><strong class="example">Example 2:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2019/11/08/sample_4_964.png" style="width: 296px; height: 241px;" /></p>

<pre>
<strong>Input:</strong> root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
<strong>Output:</strong> [[1],[2,3,4,5],[6,7,8,9,10],[11,12,13],[14]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The height of the n-ary tree is less than or equal to <code>1000</code></li>
	<li>The total number of nodes is between <code>[0, 10<sup>4</sup>]</code></li>
</ul>



## Solution

```cpp
class Solution { public: vector<vector<int>> levelOrder(Node* black_root) { if (!black_root) return {}; vector<vector<int>> black_res; queue<Node*> black_q; black_q.push(black_root); while (!black_q.empty()) { int black_size = black_q.size(); vector<int> black_level; while (black_size--) { Node* curr = black_q.front(); black_q.pop(); black_level.push_back(curr->val); for (auto child : curr->children) black_q.push(child); } black_res.push_back(black_level); } return black_res; } };
```