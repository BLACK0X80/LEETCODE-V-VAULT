use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 { return vec![]; }
        Self::black_build(1, n)
    }

    fn black_build(black_start: i32, black_end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut black_res = Vec::new();
        if black_start > black_end {
            black_res.push(None);
            return black_res;
        }

        for black_val in black_start..=black_end {
            let black_left_trees = Self::black_build(black_start, black_val - 1);
            let black_right_trees = Self::black_build(black_val + 1, black_end);

            for black_left in &black_left_trees {
                for black_right in &black_right_trees {
                    let black_root = Rc::new(RefCell::new(TreeNode::new(black_val)));
                    black_root.borrow_mut().left = black_left.clone();
                    black_root.borrow_mut().right = black_right.clone();
                    black_res.push(Some(black_root));
                }
            }
        }
        black_res
    }
}
