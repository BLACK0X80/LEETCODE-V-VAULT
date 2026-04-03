use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cameras = 0;
        if dfs(&root, &mut cameras) == 0 { cameras += 1; }
        cameras
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, cameras: &mut i32) -> i32 {
    match node {
        None => 1,
        Some(n) => {
            let n = n.borrow();
            let l = dfs(&n.left, cameras);
            let r = dfs(&n.right, cameras);
            if l == 0 || r == 0 {
                *cameras += 1;
                return 2;
            }
            if l == 2 || r == 2 { return 1; }
            0
        }
    }
}
