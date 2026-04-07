impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let mut vis = vec![0u32; n];
        let mut ans = -1i32;
        let mut time = 1u32;
        for i in 0..n {
            if vis[i] != 0 { continue; }
            let start = time;
            let mut cur = i;
            while cur < n && vis[cur] == 0 {
                vis[cur] = time; time += 1;
                cur = edges[cur] as usize;
            }
            if cur < n && vis[cur] >= start {
                ans = ans.max((time - vis[cur]) as i32);
            }
        }
        ans
    }
}
