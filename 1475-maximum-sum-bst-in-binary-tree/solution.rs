use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dfs(&root, &mut ans);
        ans
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> (bool, i32, i32, i32) {
        match node {
            None => (true, i32::MAX, i32::MIN, 0),
            Some(n) => {
                let n = n.borrow();
                let (lb, lmin, lmax, lsum) = Self::dfs(&n.left, ans);
                let (rb, rmin, rmax, rsum) = Self::dfs(&n.right, ans);
                if lb && rb && lmax < n.val && n.val < rmin {
                    let sum = lsum + rsum + n.val;
                    *ans = (*ans).max(sum);
                    (true, lmin.min(n.val), rmax.max(n.val), sum)
                } else {
                    (false, 0, 0, 0)
                }
            }
        }
    }
}
