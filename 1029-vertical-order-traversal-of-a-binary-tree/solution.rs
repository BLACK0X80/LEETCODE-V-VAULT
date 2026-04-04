use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut map: BTreeMap<i32, Vec<(i32,i32)>> = BTreeMap::new();
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, col: i32, row: i32, map: &mut BTreeMap<i32, Vec<(i32,i32)>>) {
            if let Some(n) = node {
                let n = n.borrow();
                map.entry(col).or_default().push((row, n.val));
                dfs(&n.left, col-1, row+1, map);
                dfs(&n.right, col+1, row+1, map);
            }
        }
        dfs(&root, 0, 0, &mut map);
        map.values_mut().map(|v| { v.sort(); v.iter().map(|&(_,val)| val).collect() }).collect()
    }
}
