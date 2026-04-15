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