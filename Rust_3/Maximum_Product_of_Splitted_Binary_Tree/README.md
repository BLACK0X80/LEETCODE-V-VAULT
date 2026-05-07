# Maximum Product of Splitted Binary Tree

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, split the binary tree into two subtrees by removing one edge such that the product of the sums of the subtrees is maximized.</p>

<p>Return <em>the maximum product of the sums of the two subtrees</em>. Since the answer may be too large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p><strong>Note</strong> that you need to maximize the answer before taking the mod and not after taking it.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/01/21/sample_1_1699.png" style="width: 500px; height: 167px;" />
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,6]
<strong>Output:</strong> 110
<strong>Explanation:</strong> Remove the red edge and get 2 binary trees with sum 11 and 10. Their product is 110 (11*10)
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/01/21/sample_2_1699.png" style="width: 500px; height: 211px;" />
<pre>
<strong>Input:</strong> root = [1,null,2,3,4,null,null,5,6]
<strong>Output:</strong> 90
<strong>Explanation:</strong> Remove the red edge and get 2 binary trees with sum 15 and 6.Their product is 90 (15*6)
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[2, 5 * 10<sup>4</sup>]</code>.</li>
	<li><code>1 &lt;= Node.val &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. If we know the sum of a subtree, the answer is max( (total_sum - subtree_sum) * subtree_sum) in each node.

## Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_product(black_root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut black_sums = Vec::new();
        let black_total = Self::black_get_sum(&black_root, &mut black_sums);
        let mut black_max_prod: i64 = 0;

        for black_s in black_sums {
            let bravexuneth = black_s as i64 * (black_total - black_s) as i64;
            if bravexuneth > black_max_prod {
                black_max_prod = bravexuneth;
            }
        }
        
        (black_max_prod % 1_000_000_007) as i32
    }

    fn black_get_sum(black_node: &Option<Rc<RefCell<TreeNode>>>, black_sums: &mut Vec<i32>) -> i32 {
        if let Some(black_n) = black_node {
            let black_n_borrow = black_n.borrow();
            let black_left = Self::black_get_sum(&black_n_borrow.left, black_sums);
            let black_right = Self::black_get_sum(&black_n_borrow.right, black_sums);
            let black_current_sum = black_n_borrow.val + black_left + black_right;
            black_sums.push(black_current_sum);
            black_current_sum
        } else {
            0
        }
    }
}
```