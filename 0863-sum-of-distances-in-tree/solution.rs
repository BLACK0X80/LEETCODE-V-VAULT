impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for e in &edges {
            graph[e[0] as usize].push(e[1] as usize);
            graph[e[1] as usize].push(e[0] as usize);
        }

        let mut cnt = vec![1usize; n];
        let mut ans = vec![0i64; n];

        fn dfs1(u: usize, p: usize, graph: &Vec<Vec<usize>>, cnt: &mut Vec<usize>, ans: &mut Vec<i64>) {
            for &v in &graph[u] {
                if v == p { continue; }
                dfs1(v, u, graph, cnt, ans);
                cnt[u] += cnt[v];
                ans[u] += ans[v] + cnt[v] as i64;
            }
        }

        fn dfs2(u: usize, p: usize, n: usize, graph: &Vec<Vec<usize>>, cnt: &Vec<usize>, ans: &mut Vec<i64>) {
            for &v in &graph[u] {
                if v == p { continue; }
                ans[v] = ans[u] - cnt[v] as i64 + (n - cnt[v]) as i64;
                dfs2(v, u, n, graph, cnt, ans);
            }
        }

        dfs1(0, n, &graph, &mut cnt, &mut ans);
        dfs2(0, n, n, &graph, &cnt, &mut ans);

        ans.iter().map(|&x| x as i32).collect()
    }
}
