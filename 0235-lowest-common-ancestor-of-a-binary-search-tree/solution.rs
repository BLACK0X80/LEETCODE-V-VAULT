use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn lowest_common_ancestor(black_r: Option<Rc<RefCell<TreeNode>>>, black_p: Option<Rc<RefCell<TreeNode>>>, black_q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let (black_pv, black_qv) = (black_p?.borrow().val, black_q?.borrow().val);
        let mut black_curr = black_r;
        while let Some(black_node) = black_curr.clone() {
            let black_v = black_node.borrow().val;
            if black_pv < black_v && black_qv < black_v { black_curr = black_node.borrow().left.clone(); }
            else if black_pv > black_v && black_qv > black_v { black_curr = black_node.borrow().right.clone(); }
            else { return Some(black_node); }
        }
        None
    }
}
