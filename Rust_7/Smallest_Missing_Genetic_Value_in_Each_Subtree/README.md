# Smallest Missing Genetic Value in Each Subtree

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Tree, Depth-First Search, Union-Find

---

## Problem

<p>There is a <strong>family tree</strong> rooted at <code>0</code> consisting of <code>n</code> nodes numbered <code>0</code> to <code>n - 1</code>. You are given a <strong>0-indexed</strong> integer array <code>parents</code>, where <code>parents[i]</code> is the parent for node <code>i</code>. Since node <code>0</code> is the <strong>root</strong>, <code>parents[0] == -1</code>.</p>

<p>There are <code>10<sup>5</sup></code> genetic values, each represented by an integer in the <strong>inclusive</strong> range <code>[1, 10<sup>5</sup>]</code>. You are given a <strong>0-indexed</strong> integer array <code>nums</code>, where <code>nums[i]</code> is a <strong>distinct </strong>genetic value for node <code>i</code>.</p>

<p>Return <em>an array </em><code>ans</code><em> of length </em><code>n</code><em> where </em><code>ans[i]</code><em> is</em> <em>the <strong>smallest</strong> genetic value that is <strong>missing</strong> from the subtree rooted at node</em> <code>i</code>.</p>

<p>The <strong>subtree</strong> rooted at a node <code>x</code> contains node <code>x</code> and all of its <strong>descendant</strong> nodes.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/23/case-1.png" style="width: 204px; height: 167px;" />
<pre>
<strong>Input:</strong> parents = [-1,0,0,2], nums = [1,2,3,4]
<strong>Output:</strong> [5,1,1,1]
<strong>Explanation:</strong> The answer for each subtree is calculated as follows:
- 0: The subtree contains nodes [0,1,2,3] with values [1,2,3,4]. 5 is the smallest missing value.
- 1: The subtree contains only node 1 with value 2. 1 is the smallest missing value.
- 2: The subtree contains nodes [2,3] with values [3,4]. 1 is the smallest missing value.
- 3: The subtree contains only node 3 with value 4. 1 is the smallest missing value.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/23/case-2.png" style="width: 247px; height: 168px;" />
<pre>
<strong>Input:</strong> parents = [-1,0,1,0,3,3], nums = [5,4,6,2,1,3]
<strong>Output:</strong> [7,1,1,4,2,1]
<strong>Explanation:</strong> The answer for each subtree is calculated as follows:
- 0: The subtree contains nodes [0,1,2,3,4,5] with values [5,4,6,2,1,3]. 7 is the smallest missing value.
- 1: The subtree contains nodes [1,2] with values [4,6]. 1 is the smallest missing value.
- 2: The subtree contains only node 2 with value 6. 1 is the smallest missing value.
- 3: The subtree contains nodes [3,4,5] with values [2,1,3]. 4 is the smallest missing value.
- 4: The subtree contains only node 4 with value 1. 2 is the smallest missing value.
- 5: The subtree contains only node 5 with value 3. 1 is the smallest missing value.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> parents = [-1,2,3,0,2,4,1], nums = [2,3,4,5,6,7,8]
<strong>Output:</strong> [1,1,1,1,1,1,1]
<strong>Explanation:</strong> The value 1 is missing from all the subtrees.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == parents.length == nums.length</code></li>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= parents[i] &lt;= n - 1</code> for <code>i != 0</code></li>
	<li><code>parents[0] == -1</code></li>
	<li><code>parents</code> represents a valid tree.</li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li>Each <code>nums[i]</code> is distinct.</li>
</ul>


## Hints

1. If the subtree doesn't contain 1, then the missing value will always be 1.
2. What data structure allows us to dynamically update the values that are currently not present?

## Solution

```rust
impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let n = parents.len();
        let mut ans = vec![1; n];
        let mut node_with_1 = None;
        for (i, &v) in nums.iter().enumerate() {
            if v == 1 { node_with_1 = Some(i); break; }
        }

        let u_1 = match node_with_1 {
            Some(idx) => idx,
            None => return ans,
        };

        let mut children = vec![vec![]; n];
        for (i, &p) in parents.iter().enumerate() {
            if p != -1 { children[p as usize].push(i); }
        }

        let mut seen_vals = vec![false; 100010];
        let mut visited_nodes = vec![false; n];
        let mut miss = 1;

        fn fill(u: usize, children: &Vec<Vec<usize>>, visited: &mut Vec<bool>, seen: &mut Vec<bool>, nums: &Vec<i32>) {
            if visited[u] { return; }
            visited[u] = true;
            seen[nums[u] as usize] = true;
            for &v in &children[u] {
                fill(v, children, visited, seen, nums);
            }
        }

        let mut curr: i32 = u_1 as i32;
        while curr != -1 {
            let u = curr as usize;
            fill(u, &children, &mut visited_nodes, &mut seen_vals, &nums);
            while seen_vals[miss] { miss += 1; }
            ans[u] = miss as i32;
            curr = parents[u];
        }
        ans
    }
}
```