use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dfs(&root, &mut ans);
        ans
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        let node = match node { Some(n) => n, None => return 0 };
        let n = node.borrow();
        let l = Self::dfs(&n.left, ans);
        let r = Self::dfs(&n.right, ans);
        let lp = if n.left.as_ref().map_or(false, |x| x.borrow().val == n.val) { l + 1 } else { 0 };
        let rp = if n.right.as_ref().map_or(false, |x| x.borrow().val == n.val) { r + 1 } else { 0 };
        *ans = (*ans).max(lp + rp);
        lp.max(rp)
    }
}