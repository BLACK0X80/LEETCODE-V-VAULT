use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
            Some(node) => {
                if val < node.borrow().val {
                    let left = node.borrow().left.clone();
                    node.borrow_mut().left = Self::insert_into_bst(left, val);
                } else {
                    let right = node.borrow().right.clone();
                    node.borrow_mut().right = Self::insert_into_bst(right, val);
                }
                Some(node)
            }
        }
    }
}