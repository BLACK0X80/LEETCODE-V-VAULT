use std::rc::Rc; use std::cell::RefCell;
impl Solution { pub fn tree2str(black_root: Option<Rc<RefCell<TreeNode>>>) -> String {
    match black_root {
        None => String::new(),
        Some(node) => {
            let n = node.borrow();
            let mut res = n.val.to_string();
            if n.left.is_some() || n.right.is_some() {
                res += &format!("({})", Self::tree2str(n.left.clone()));
            }
            if n.right.is_some() {
                res += &format!("({})", Self::tree2str(n.right.clone()));
            }
            res
        }
    }
} }