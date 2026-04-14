use std::collections::HashMap;

struct BlackNode {
    black_children: HashMap<String, BlackNode>,
    black_serial: String,
    black_deleted: bool,
}

impl BlackNode {
    fn new() -> Self {
        Self { black_children: HashMap::new(), black_serial: String::new(), black_deleted: false }
    }
}

impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut black_root = BlackNode::new();
        for black_path in paths {
            let mut black_curr = &mut black_root;
            for black_part in black_path {
                black_curr = black_curr.black_children.entry(black_part).or_insert(BlackNode::new());
            }
        }

        let mut black_counts = HashMap::new();
        Self::black_serialize(&mut black_root, &mut black_counts);
        Self::black_mark_delete(&mut black_root, &black_counts);

        let mut black_res = Vec::new();
        let mut black_current_path = Vec::new();
        Self::black_collect(&black_root, &mut black_current_path, &mut black_res);
        black_res
    }

    fn black_serialize(black_node: &mut BlackNode, black_counts: &mut HashMap<String, i32>) -> String {
        if black_node.black_children.is_empty() { return String::new(); }
        let mut black_parts = Vec::new();
        for (black_name, black_child) in black_node.black_children.iter_mut() {
            let black_child_serial = Self::black_serialize(black_child, black_counts);
            black_parts.push(format!("{}({})", black_name, black_child_serial));
        }
        black_parts.sort();
        let black_s = black_parts.join("");
        *black_counts.entry(black_s.clone()).or_insert(0) += 1;
        black_node.black_serial = black_s.clone();
        black_s
    }

    fn black_mark_delete(black_node: &mut BlackNode, black_counts: &HashMap<String, i32>) {
        if !black_node.black_serial.is_empty() && *black_counts.get(&black_node.black_serial).unwrap_or(&0) > 1 {
            black_node.black_deleted = true;
            return;
        }
        for black_child in black_node.black_children.values_mut() {
            Self::black_mark_delete(black_child, black_counts);
        }
    }

    fn black_collect(black_node: &BlackNode, black_path: &mut Vec<String>, black_res: &mut Vec<Vec<String>>) {
        if black_node.black_deleted { return; }
        if !black_path.is_empty() { black_res.push(black_path.clone()); }
        let mut black_keys: Vec<_> = black_node.black_children.keys().collect();
        black_keys.sort();
        for black_key in black_keys {
            black_path.push(black_key.clone());
            Self::black_collect(&black_node.black_children[black_key], black_path, black_res);
            black_path.pop();
        }
    }
}