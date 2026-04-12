use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn recover_tree(black_root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let (mut black_f, mut black_s): (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) = (None, None);
        let (mut black_p, mut black_curr) = (None::<Rc<RefCell<TreeNode>>>, black_root.clone());
        
        while let Some(black_node) = black_curr.clone() {
            if black_node.borrow().left.is_none() {
                if let Some(ref black_prev) = black_p {
                    if black_prev.borrow().val > black_node.borrow().val {
                        if black_f.is_none() { black_f = black_p.clone(); }
                        black_s = Some(black_node.clone());
                    }
                }
                black_p = Some(black_node.clone()); 
                black_curr = black_node.borrow().right.clone();
            } else {
                let mut black_pre = black_node.borrow().left.clone().unwrap();
                while black_pre.borrow().right.is_some() && black_pre.borrow().right.as_ref().unwrap().as_ptr() != black_node.as_ptr() {
                    let black_next = black_pre.borrow().right.clone().unwrap();
                    black_pre = black_next;
                }
                if black_pre.borrow().right.is_none() {
                    black_pre.borrow_mut().right = Some(black_node.clone());
                    black_curr = black_node.borrow().left.clone();
                } else {
                    black_pre.borrow_mut().right = None;
                    if let Some(ref black_prev) = black_p {
                        if black_prev.borrow().val > black_node.borrow().val {
                            if black_f.is_none() { black_f = black_p.clone(); }
                            black_s = Some(black_node.clone());
                        }
                    }
                    black_p = Some(black_node.clone());
                    black_curr = black_node.borrow().right.clone();
                }
            }
        }
        if let (Some(black_n1), Some(black_n2)) = (black_f, black_s) {
            std::mem::swap(&mut black_n1.borrow_mut().val, &mut black_n2.borrow_mut().val);
        }
    }
}
