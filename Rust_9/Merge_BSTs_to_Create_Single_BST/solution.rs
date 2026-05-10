use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_merge(black1: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut black2 = HashMap::new();
        let mut black3 = HashSet::new();
        for black4 in &black1 {
            if let Some(n) = black4 {
                black2.insert(n.borrow().val, n.clone());
                if let Some(ref l) = n.borrow().left { black3.insert(l.borrow().val); }
                if let Some(ref r) = n.borrow().right { black3.insert(r.borrow().val); }
            }
        }
        let mut black5 = None;
        for black4 in &black1 {
            if let Some(n) = black4 {
                if !black3.contains(&n.borrow().val) {
                    if black5.is_some() { return None; }
                    black5 = Some(n.clone());
                }
            }
        }
        let black6 = black5?;
        let mut black7 = 1;
        fn solve(n: Option<Rc<RefCell<TreeNode>>>, m: &mut HashMap<i32, Rc<RefCell<TreeNode>>>, c: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            let curr = n?;
            let (l, r, v) = {
                let b = curr.borrow();
                (b.left.clone(), b.right.clone(), b.val)
            };
            if l.is_none() && r.is_none() {
                if let Some(next) = m.remove(&v) {
                    *c += 1;
                    curr.borrow_mut().left = solve(next.borrow().left.clone(), m, c);
                    curr.borrow_mut().right = solve(next.borrow().right.clone(), m, c);
                }
            } else {
                curr.borrow_mut().left = solve(l, m, c);
                curr.borrow_mut().right = solve(r, m, c);
            }
            Some(curr)
        }
        black2.remove(&black6.borrow().val);
        let black8 = solve(Some(black6), &mut black2, &mut black7);
        if black7 != black1.len() { return None; }
        fn valid(n: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
            if let Some(node) = n {
                let v = node.borrow().val as i64;
                v > min && v < max && valid(node.borrow().left.clone(), min, v) && valid(node.borrow().right.clone(), v, max)
            } else { true }
        }
        if valid(black8.clone(), i64::MIN, i64::MAX) { black8 } else { None }
    }
}