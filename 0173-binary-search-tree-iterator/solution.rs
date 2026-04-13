use std::{rc::Rc, cell::RefCell};
struct BSTIterator { black_v: Vec<i32>, black_i: usize }
impl BSTIterator {
    fn new(black_r: Option<Rc<RefCell<TreeNode>>>) -> Self { let mut black_v = vec![]; Self::black_h(black_r, &mut black_v); Self { black_v, black_i: 0 } }
    fn black_h(black_n: Option<Rc<RefCell<TreeNode>>>, black_v: &mut Vec<i32>) { if let Some(black_node) = black_n { let black_b = black_node.borrow(); Self::black_h(black_b.left.clone(), black_v); black_v.push(black_b.val); Self::black_h(black_b.right.clone(), black_v); } }
    fn next(&mut self) -> i32 { let black_val = self.black_v[self.black_i]; self.black_i += 1; black_val }
    fn has_next(&self) -> bool { self.black_i < self.black_v.len() }
}
