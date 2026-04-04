use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = HashMap::new();
        for (i, &v) in inorder.iter().enumerate() { map.insert(v, i); }
        Self::build(&inorder, &postorder, 0, inorder.len(), 0, postorder.len(), &map)
    }
    fn build(inorder: &[i32], postorder: &[i32], il: usize, ir: usize, pl: usize, pr: usize, map: &HashMap<i32, usize>) -> Option<Rc<RefCell<TreeNode>>> {
        if il >= ir || pl >= pr { return None; }
        let root_val = postorder[pr - 1];
        let idx = map[&root_val];
        let left_len = idx - il;
        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left: Self::build(inorder, postorder, il, idx, pl, pl + left_len, map),
            right: Self::build(inorder, postorder, idx + 1, ir, pl + left_len, pr - 1, map),
        })))
    }
}
