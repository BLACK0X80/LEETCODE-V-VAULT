use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(root).1
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        match node {
            None => (0, None),
            Some(n) => {
                let (black_l_d, black_l_n) = Self::dfs(n.borrow().left.clone());
                let (black_r_d, black_r_n) = Self::dfs(n.borrow().right.clone());
                if black_l_d > black_r_d { (black_l_d + 1, black_l_n) }
                else if black_r_d > black_l_d { (black_r_d + 1, black_r_n) }
                else { (black_l_d + 1, Some(n)) }
            }
        }
    }
}