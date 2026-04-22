# All Nodes Distance K in Binary Tree

**Difficulty:** Medium
**Tags:** Hash Table, Tree, Depth-First Search, Breadth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, the value of a target node <code>target</code>, and an integer <code>k</code>, return <em>an array of the values of all nodes that have a distance </em><code>k</code><em> from the target node.</em></p>

<p>You can return the answer in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/06/28/sketch0.png" style="width: 500px; height: 429px;" />
<pre>
<strong>Input:</strong> root = [3,5,1,6,2,0,8,null,null,7,4], target = 5, k = 2
<strong>Output:</strong> [7,4,1]
Explanation: The nodes that are a distance 2 from the target node (with value 5) have values 7, 4, and 1.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> root = [1], target = 1, k = 3
<strong>Output:</strong> []
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 500]</code>.</li>
	<li><code>0 &lt;= Node.val &lt;= 500</code></li>
	<li>All the values <code>Node.val</code> are <strong>unique</strong>.</li>
	<li><code>target</code> is the value of one of the nodes in the tree.</li>
	<li><code>0 &lt;= k &lt;= 1000</code></li>
</ul>



## Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
        let mut black_adj = HashMap::new();
        Self::build_adj(root, None, &mut black_adj);
        
        let target_val = target.unwrap().borrow().val;
        let mut black_q = VecDeque::from([(target_val, 0)]);
        let mut black_vis = HashSet::from([target_val]);
        let mut black_res = vec![];

        while let Some((val, dist)) = black_q.pop_front() {
            if dist == k {
                black_res.push(val);
            } else if dist < k {
                if let Some(neighbors) = black_adj.get(&val) {
                    for &next in neighbors {
                        if black_vis.insert(next) {
                            black_q.push_back((next, dist + 1));
                        }
                    }
                }
            }
        }
        black_res
    }

    fn build_adj(node: Option<Rc<RefCell<TreeNode>>>, parent: Option<i32>, adj: &mut HashMap<i32, Vec<i32>>) {
        if let Some(n) = node {
            let val = n.borrow().val;
            if let Some(p_val) = parent {
                adj.entry(val).or_default().push(p_val);
                adj.entry(p_val).or_default().push(val);
            }
            Self::build_adj(n.borrow().left.clone(), Some(val), adj);
            Self::build_adj(n.borrow().right.clone(), Some(val), adj);
        }
    }
}
```