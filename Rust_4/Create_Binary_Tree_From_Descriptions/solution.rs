use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
        let mut children = std::collections::HashSet::new();
        for d in &descriptions {
            let (p, c, left) = (d[0], d[1], d[2] == 1);
            let pn = nodes.entry(p).or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(p)))).clone();
            let cn = nodes.entry(c).or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(c)))).clone();
            if left { pn.borrow_mut().left = Some(cn); } else { pn.borrow_mut().right = Some(cn); }
            children.insert(c);
        }
        nodes.into_iter().find(|(k, _)| !children.contains(k)).map(|(_, v)| v)
    }
}