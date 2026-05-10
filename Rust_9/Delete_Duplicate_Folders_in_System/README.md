# Delete Duplicate Folders in System

**Difficulty:** Hard
**Tags:** Array, Hash Table, String, Trie, Hash Function

---

## Problem

<p>Due to a bug, there are many duplicate folders in a file system. You are given a 2D array <code>paths</code>, where <code>paths[i]</code> is an array representing an absolute path to the <code>i<sup>th</sup></code> folder in the file system.</p>

<ul>
	<li>For example, <code>[&quot;one&quot;, &quot;two&quot;, &quot;three&quot;]</code> represents the path <code>&quot;/one/two/three&quot;</code>.</li>
</ul>

<p>Two folders (not necessarily on the same level) are <strong>identical</strong> if they contain the <strong>same non-empty</strong> set of identical subfolders and underlying subfolder structure. The folders <strong>do not</strong> need to be at the root level to be identical. If two or more folders are <strong>identical</strong>, then <strong>mark</strong> the folders as well as all their subfolders.</p>

<ul>
	<li>For example, folders <code>&quot;/a&quot;</code> and <code>&quot;/b&quot;</code> in the file structure below are identical. They (as well as their subfolders) should <strong>all</strong> be marked:

	<ul>
		<li><code>/a</code></li>
		<li><code>/a/x</code></li>
		<li><code>/a/x/y</code></li>
		<li><code>/a/z</code></li>
		<li><code>/b</code></li>
		<li><code>/b/x</code></li>
		<li><code>/b/x/y</code></li>
		<li><code>/b/z</code></li>
	</ul>
	</li>
	<li>However, if the file structure also included the path <code>&quot;/b/w&quot;</code>, then the folders <code>&quot;/a&quot;</code> and <code>&quot;/b&quot;</code> would not be identical. Note that <code>&quot;/a/x&quot;</code> and <code>&quot;/b/x&quot;</code> would still be considered identical even with the added folder.</li>
</ul>

<p>Once all the identical folders and their subfolders have been marked, the file system will <strong>delete</strong> all of them. The file system only runs the deletion once, so any folders that become identical after the initial deletion are not deleted.</p>

<p>Return <em>the 2D array </em><code>ans</code> <em>containing the paths of the <strong>remaining</strong> folders after deleting all the marked folders. The paths may be returned in <strong>any</strong> order</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/07/19/lc-dupfolder1.jpg" style="width: 200px; height: 218px;" />
<pre>
<strong>Input:</strong> paths = [[&quot;a&quot;],[&quot;c&quot;],[&quot;d&quot;],[&quot;a&quot;,&quot;b&quot;],[&quot;c&quot;,&quot;b&quot;],[&quot;d&quot;,&quot;a&quot;]]
<strong>Output:</strong> [[&quot;d&quot;],[&quot;d&quot;,&quot;a&quot;]]
<strong>Explanation:</strong> The file structure is as shown.
Folders &quot;/a&quot; and &quot;/c&quot; (and their subfolders) are marked for deletion because they both contain an empty
folder named &quot;b&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/07/19/lc-dupfolder2.jpg" style="width: 200px; height: 355px;" />
<pre>
<strong>Input:</strong> paths = [[&quot;a&quot;],[&quot;c&quot;],[&quot;a&quot;,&quot;b&quot;],[&quot;c&quot;,&quot;b&quot;],[&quot;a&quot;,&quot;b&quot;,&quot;x&quot;],[&quot;a&quot;,&quot;b&quot;,&quot;x&quot;,&quot;y&quot;],[&quot;w&quot;],[&quot;w&quot;,&quot;y&quot;]]
<strong>Output:</strong> [[&quot;c&quot;],[&quot;c&quot;,&quot;b&quot;],[&quot;a&quot;],[&quot;a&quot;,&quot;b&quot;]]
<strong>Explanation: </strong>The file structure is as shown. 
Folders &quot;/a/b/x&quot; and &quot;/w&quot; (and their subfolders) are marked for deletion because they both contain an empty folder named &quot;y&quot;.
Note that folders &quot;/a&quot; and &quot;/c&quot; are identical after the deletion, but they are not deleted because they were not marked beforehand.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/07/19/lc-dupfolder3.jpg" style="width: 200px; height: 201px;" />
<pre>
<strong>Input:</strong> paths = [[&quot;a&quot;,&quot;b&quot;],[&quot;c&quot;,&quot;d&quot;],[&quot;c&quot;],[&quot;a&quot;]]
<strong>Output:</strong> [[&quot;c&quot;],[&quot;c&quot;,&quot;d&quot;],[&quot;a&quot;],[&quot;a&quot;,&quot;b&quot;]]
<strong>Explanation:</strong> All folders are unique in the file system.
Note that the returned array can be in a different order as the order does not matter.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= paths.length &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= paths[i].length &lt;= 500</code></li>
	<li><code>1 &lt;= paths[i][j].length &lt;= 10</code></li>
	<li><code>1 &lt;= sum(paths[i][j].length) &lt;= 2 * 10<sup>5</sup></code></li>
	<li><code>path[i][j]</code> consists of lowercase English letters.</li>
	<li>No two paths lead to the same folder.</li>
	<li>For any folder not at the root level, its parent folder will also be in the input.</li>
</ul>


## Hints

1. Can we use a trie to build the folder structure?
2. Can we utilize hashing to hash the folder structures?

## Solution

```rust
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
```