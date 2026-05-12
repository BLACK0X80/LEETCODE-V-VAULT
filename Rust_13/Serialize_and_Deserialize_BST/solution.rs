use std::rc::Rc; use std::cell::RefCell;
struct Codec {}
impl Codec {
    fn new() -> Self { Codec {} }
    fn serialize(&self, black_root: Option<Rc<RefCell<TreeNode>>>) -> String { let mut black_res = vec![]; fn pre(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<String>) { if let Some(n) = node { let n = n.borrow(); res.push(n.val.to_string()); pre(n.left.clone(), res); pre(n.right.clone(), res); } } pre(black_root, &mut black_res); black_res.join(",") }
    fn deserialize(&self, black_data: String) -> Option<Rc<RefCell<TreeNode>>> { if black_data.is_empty() { return None; } let mut black_vals: std::collections::VecDeque<i32> = black_data.split(',').map(|s| s.parse().unwrap()).collect(); fn build(vals: &mut std::collections::VecDeque<i32>, min: i32, max: i32) -> Option<Rc<RefCell<TreeNode>>> { if let Some(&v) = vals.front() { if v > min && v < max { vals.pop_front(); let mut node = TreeNode::new(v); node.left = build(vals, min, v); node.right = build(vals, v, max); return Some(Rc::new(RefCell::new(node))); } } None } build(&mut black_vals, i32::MIN, i32::MAX) }
}