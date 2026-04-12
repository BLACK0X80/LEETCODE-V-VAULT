use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn flatten(black_root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut black_curr = black_root.clone();
        while let Some(black_node) = black_curr {
            let mut black_b = black_node.borrow_mut();
            if let Some(black_left) = black_b.left.take() {
                let mut black_pre = black_left.clone();
                while let Some(black_next) = {
                    let black_p = black_pre.borrow();
                    black_p.right.clone()
                } { black_pre = black_next; }
                black_pre.borrow_mut().right = black_b.right.take();
                black_b.right = Some(black_left);
            }
            black_curr = black_b.right.clone();
        }
    }
}
