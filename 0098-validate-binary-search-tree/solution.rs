use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_valid_bst(black_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn black_check(black_node: &Option<Rc<RefCell<TreeNode>>>, black_min: Option<i64>, black_max: Option<i64>) -> bool {
            if let Some(black_n) = black_node {
                let black_v = black_n.borrow().val as i64;
                if black_min.map_or(false, |m| black_v <= m) || black_max.map_or(false, |m| black_v >= m) { return false; }
                black_check(&black_n.borrow().left, black_min, Some(black_v)) && 
                black_check(&black_n.borrow().right, Some(black_v), black_max)
            } else { true }
        }
        black_check(&black_root, None, None)
    }
}
