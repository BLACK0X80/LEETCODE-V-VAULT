use std::rc::Rc;
use std::cell::RefCell;

struct Codec;

impl Codec {
    fn new() -> Self {
        Codec
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = Vec::new();
        Self::serialize_helper(&root, &mut result);
        result.join(",")
    }

    fn serialize_helper(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<String>) {
        match node {
            None => result.push("N".to_string()),
            Some(n) => {
                let n = n.borrow();
                result.push(n.val.to_string());
                Self::serialize_helper(&n.left, result);
                Self::serialize_helper(&n.right, result);
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut tokens: Vec<&str> = data.split(',').collect();
        tokens.reverse();
        Self::deserialize_helper(&mut tokens)
    }

    fn deserialize_helper(tokens: &mut Vec<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        let token = tokens.pop()?;
        if token == "N" {
            return None;
        }
        let val: i32 = token.parse().unwrap();
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        node.borrow_mut().left = Self::deserialize_helper(tokens);
        node.borrow_mut().right = Self::deserialize_helper(tokens);
        Some(node)
    }
}
