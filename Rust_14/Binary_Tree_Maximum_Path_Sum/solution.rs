use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = i32::MIN;
        Self::dfs(&root, &mut max);
        max
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        let node = match node {
            None => return 0,
            Some(n) => n.borrow(),
        };

        let left  = Self::dfs(&node.left,  max).max(0);
        let right = Self::dfs(&node.right, max).max(0);

        *max = (*max).max(node.val + left + right);

        node.val + left.max(right)
    }
}