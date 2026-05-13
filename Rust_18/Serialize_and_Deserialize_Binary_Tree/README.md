# Serialize and Deserialize Binary Tree

**Difficulty:** Hard
**Tags:** String, Tree, Depth-First Search, Breadth-First Search, Design, Binary Tree

---

## Problem

<p>Serialization is the process of converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.</p>

<p>Design an algorithm to serialize and deserialize a binary tree. There is no restriction on how your serialization/deserialization algorithm should work. You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.</p>

<p><strong>Clarification:</strong> The input/output format is the same as <a href="https://support.leetcode.com/hc/en-us/articles/32442719377939-How-to-create-test-cases-on-LeetCode#h_01J5EGREAW3NAEJ14XC07GRW1A" target="_blank">how LeetCode serializes a binary tree</a>. You do not necessarily need to follow this format, so please be creative and come up with different approaches yourself.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/serdeser.jpg" style="width: 442px; height: 324px;" />
<pre>
<strong>Input:</strong> root = [1,2,3,null,null,4,5]
<strong>Output:</strong> [1,2,3,null,null,4,5]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> root = []
<strong>Output:</strong> []
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[0, 10<sup>4</sup>]</code>.</li>
	<li><code>-1000 &lt;= Node.val &lt;= 1000</code></li>
</ul>



## Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Codec;

impl Codec {
    fn new() -> Self {
        Codec
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = Vec::new();
        Self::serialize_helper(&root, &mut result);
        result.join(",")
    }

    fn serialize_helper(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<String>) {
        match node {
            None => result.push("N".to_string()),
            Some(n) => {
                let n = n.borrow();
                result.push(n.val.to_string());
                Self::serialize_helper(&n.left, result);
                Self::serialize_helper(&n.right, result);
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut tokens: Vec<&str> = data.split(',').collect();
        tokens.reverse();
        Self::deserialize_helper(&mut tokens)
    }

    fn deserialize_helper(tokens: &mut Vec<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        let token = tokens.pop()?;
        if token == "N" {
            return None;
        }
        let val: i32 = token.parse().unwrap();
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        node.borrow_mut().left = Self::deserialize_helper(tokens);
        node.borrow_mut().right = Self::deserialize_helper(tokens);
        Some(node)
    }
}
```