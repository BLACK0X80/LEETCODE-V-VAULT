use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let b = traversal.as_bytes();
        let mut i = 0;
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();

        while i < b.len() {
            let mut depth = 0;
            while i < b.len() && b[i] == b'-' { depth += 1; i += 1; }
            let mut val = 0;
            while i < b.len() && b[i].is_ascii_digit() { val = val * 10 + (b[i] - b'0') as i32; i += 1; }

            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            while stack.len() > depth { stack.pop(); }

            if let Some(parent) = stack.last() {
                let mut p = parent.borrow_mut();
                if p.left.is_none() { p.left = Some(node.clone()); }
                else { p.right = Some(node.clone()); }
            }
            stack.push(node);
        }

        stack.into_iter().next()
    }
}
