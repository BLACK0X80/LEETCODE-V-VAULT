use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                let v = node.borrow().val;
                if key < v {
                    let left = node.borrow().left.clone();
                    node.borrow_mut().left = Self::delete_node(left, key);
                    Some(node)
                } else if key > v {
                    let right = node.borrow().right.clone();
                    node.borrow_mut().right = Self::delete_node(right, key);
                    Some(node)
                } else {
                    let left = node.borrow().left.clone();
                    let right = node.borrow().right.clone();
                    match (left, right) {
                        (None, r) => r,
                        (l, None) => l,
                        (l, r) => {
                            let mut cur = r.clone();
                            loop {
                                let next = cur.as_ref().unwrap().borrow().left.clone();
                                if next.is_none() { break; }
                                cur = next;
                            }
                            cur.as_ref().unwrap().borrow_mut().left = l;
                            r
                        }
                    }
                }
            }
        }
    }
}