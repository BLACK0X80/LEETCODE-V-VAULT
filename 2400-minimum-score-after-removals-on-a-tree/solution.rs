impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut adj = vec![vec![]; n];
        for e in edges {
            adj[e[0] as usize].push(e[1] as usize);
            adj[e[1] as usize].push(e[0] as usize);
        }

        let mut sub_xor = vec![0; n];
        let mut entry = vec![0; n];
        let mut exit = vec![0; n];
        let mut timer = 0;

        fn dfs(u: usize, p: usize, timer: &mut i32, adj: &Vec<Vec<usize>>, nums: &Vec<i32>, sub_xor: &mut Vec<i32>, entry: &mut Vec<i32>, exit: &mut Vec<i32>) {
            *timer += 1;
            entry[u] = *timer;
            sub_xor[u] = nums[u];
            for &v in &adj[u] {
                if v != p {
                    dfs(v, u, timer, adj, nums, sub_xor, entry, exit);
                    sub_xor[u] ^= sub_xor[v];
                }
            }
            exit[u] = *timer;
        }

        dfs(0, 0, &mut timer, &adj, &nums, &mut sub_xor, &mut entry, &mut exit);
        let total_xor = sub_xor[0];
        let mut ans = i32::MAX;

        let is_ancestor = |u: usize, v: usize| -> bool {
            entry[u] <= entry[v] && exit[u] >= exit[v]
        };

        for i in 1..n {
            for j in i + 1..n {
                let (mut a, mut b, mut c) = (0, 0, 0);
                if is_ancestor(i, j) {
                    c = sub_xor[j];
                    b = sub_xor[i] ^ c;
                    a = total_xor ^ sub_xor[i];
                } else if is_ancestor(j, i) {
                    c = sub_xor[i];
                    b = sub_xor[j] ^ c;
                    a = total_xor ^ sub_xor[j];
                } else {
                    a = total_xor ^ sub_xor[i] ^ sub_xor[j];
                    b = sub_xor[i];
                    c = sub_xor[j];
                }
                let max_v = a.max(b.max(c));
                let min_v = a.min(b.min(c));
                ans = ans.min(max_v - min_v);
            }
        }
        ans
    }
}
