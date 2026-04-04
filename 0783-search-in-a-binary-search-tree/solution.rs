use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                let v = node.borrow().val;
                if v == val { Some(node) }
                else if val < v { Self::search_bst(node.borrow().left.clone(), val) }
                else { Self::search_bst(node.borrow().right.clone(), val) }
            }
        }
    }
}
