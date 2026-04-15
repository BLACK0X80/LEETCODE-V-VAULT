use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut black1 = HashMap::new();
        let mut black2 = 0;
        fn dfs1(n: &Option<Rc<RefCell<TreeNode>>>, d: i32, h: &mut HashMap<i32, i32>) -> i32 {
            if let Some(node) = n {
                let node = node.borrow();
                let res = 1 + dfs1(&node.left, d + 1, h).max(dfs1(&node.right, d + 1, h));
                h.insert(node.val, res);
                res
            } else { -1 }
        }
        dfs1(&root, 0, &mut black1);
        let mut black3 = HashMap::new();
        fn dfs2(n: &Option<Rc<RefCell<TreeNode>>>, d: i32, cur: i32, h: &HashMap<i32, i32>, res: &mut HashMap<i32, i32>) {
            if let Some(node) = n {
                let node = node.borrow();
                res.insert(node.val, cur);
                let lh = node.left.as_ref().map_or(-1, |l| h[&l.borrow().val]);
                let rh = node.right.as_ref().map_or(-1, |r| h[&r.borrow().val]);
                dfs2(&node.left, d + 1, cur.max(d + 1 + rh), h, res);
                dfs2(&node.right, d + 1, cur.max(d + 1 + lh), h, res);
            }
        }
        dfs2(&root, 0, -1, &black1, &mut black3);
        queries.into_iter().map(|q| black3[&q]).collect()
    }
}