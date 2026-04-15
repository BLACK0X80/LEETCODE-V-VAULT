# Populating Next Right Pointers in Each Node II

**Difficulty:** Medium
**Tags:** Linked List, Tree, Depth-First Search, Breadth-First Search, Binary Tree

---

## Problem

<p>Given a binary tree</p>

<pre>
struct Node {
  int val;
  Node *left;
  Node *right;
  Node *next;
}
</pre>

<p>Populate each next pointer to point to its next right node. If there is no next right node, the next pointer should be set to <code>NULL</code>.</p>

<p>Initially, all next pointers are set to <code>NULL</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/02/15/117_sample.png" style="width: 500px; height: 171px;" />
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,null,7]
<strong>Output:</strong> [1,#,2,3,#,4,5,7,#]
<strong>Explanation: </strong>Given the above binary tree (Figure A), your function should populate each next pointer to point to its next right node, just like in Figure B. The serialized output is in level order as connected by the next pointers, with &#39;#&#39; signifying the end of each level.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> root = []
<strong>Output:</strong> []
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[0, 6000]</code>.</li>
	<li><code>-100 &lt;= Node.val &lt;= 100</code></li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow-up:</strong></p>

<ul>
	<li>You may only use constant extra space.</li>
	<li>The recursive approach is fine. You may assume implicit stack space does not count as extra space for this problem.</li>
</ul>



## Solution

```cpp
static const int black_fast = [](){ios_base::sync_with_stdio(0);cin.tie(0);return 0;}();
class Solution { public: Node* connect(Node* black_r) { Node *black_c=black_r; while(black_c){ Node black_d(0); Node *black_t=&black_d; while(black_c){ if(black_c->left){ black_t->next=black_c->left; black_t=black_t->next; } if(black_c->right){ black_t->next=black_c->right; black_t=black_t->next; } black_c=black_c->next; } black_c=black_d.next; } return black_r; } };
```