impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let n = parent.len();
        let b = s.as_bytes();
        let mut children = vec![vec![]; n];
        for i in 1..n { children[parent[i] as usize].push(i); }
        let mut ans = 1;
        fn dfs(u: usize, children: &Vec<Vec<usize>>, b: &[u8], ans: &mut i32) -> i32 {
            let mut top2 = [0i32; 2];
            for &v in &children[u] {
                let d = dfs(v, children, b, ans);
                if b[v] != b[u] {
                    if d > top2[0] { top2[1]=top2[0]; top2[0]=d; }
                    else if d > top2[1] { top2[1]=d; }
                }
            }
            *ans = (*ans).max(top2[0]+top2[1]+1);
            top2[0]+1
        }
        dfs(0, &children, b, &mut ans);
        ans
    }
}
