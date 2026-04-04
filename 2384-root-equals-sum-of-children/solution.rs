use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let r = root.unwrap();
        let r = r.borrow();
        r.val == r.left.as_ref().unwrap().borrow().val + r.right.as_ref().unwrap().borrow().val
    }
}
