use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
        let mut black_adj = HashMap::new();
        Self::build_adj(root, None, &mut black_adj);
        
        let target_val = target.unwrap().borrow().val;
        let mut black_q = VecDeque::from([(target_val, 0)]);
        let mut black_vis = HashSet::from([target_val]);
        let mut black_res = vec![];

        while let Some((val, dist)) = black_q.pop_front() {
            if dist == k {
                black_res.push(val);
            } else if dist < k {
                if let Some(neighbors) = black_adj.get(&val) {
                    for &next in neighbors {
                        if black_vis.insert(next) {
                            black_q.push_back((next, dist + 1));
                        }
                    }
                }
            }
        }
        black_res
    }

    fn build_adj(node: Option<Rc<RefCell<TreeNode>>>, parent: Option<i32>, adj: &mut HashMap<i32, Vec<i32>>) {
        if let Some(n) = node {
            let val = n.borrow().val;
            if let Some(p_val) = parent {
                adj.entry(val).or_default().push(p_val);
                adj.entry(p_val).or_default().push(val);
            }
            Self::build_adj(n.borrow().left.clone(), Some(val), adj);
            Self::build_adj(n.borrow().right.clone(), Some(val), adj);
        }
    }
}