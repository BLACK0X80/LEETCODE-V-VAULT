impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = graph.len() - 1;
        let mut res = Vec::new();
        let mut path = vec![0];
        fn dfs(u: usize, n: usize, path: &mut Vec<i32>, graph: &Vec<Vec<i32>>, res: &mut Vec<Vec<i32>>) {
            if u == n { res.push(path.clone()); return; }
            for &v in &graph[u] {
                path.push(v);
                dfs(v as usize, n, path, graph, res);
                path.pop();
            }
        }
        dfs(0, n, &mut path, &graph, &mut res);
        res
    }
}