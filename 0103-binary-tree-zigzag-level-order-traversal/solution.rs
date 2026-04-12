use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn zigzag_level_order(black_root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut black_res = vec![];
        fn black_dfs(black_node: &Option<Rc<RefCell<TreeNode>>>, black_lvl: usize, black_res: &mut Vec<Vec<i32>>) {
            if let Some(black_n) = black_node {
                if black_res.len() == black_lvl { black_res.push(vec![]); }
                black_res[black_lvl].push(black_n.borrow().val);
                black_dfs(&black_n.borrow().left, black_lvl + 1, black_res);
                black_dfs(&black_n.borrow().right, black_lvl + 1, black_res);
            }
        }
        black_dfs(&black_root, 0, &mut black_res);
        for black_i in 0..black_res.len() {
            if black_i % 2 == 1 { black_res[black_i].reverse(); }
        }
        black_res
    }
}
