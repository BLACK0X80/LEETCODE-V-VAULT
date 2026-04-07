impl Solution {
    pub fn component_value(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut black1 = vec![vec![]; n];
        for e in edges {
            black1[e[0] as usize].push(e[1] as usize);
            black1[e[1] as usize].push(e[0] as usize);
        }
        let black2: i32 = nums.iter().sum();
        fn dfs(u: usize, p: usize, t: i32, g: &Vec<Vec<usize>>, v: &Vec<i32>) -> i32 {
            let mut black3 = v[u];
            for &next in &g[u] {
                if next != p {
                    let res = dfs(next, u, t, g, v);
                    if res == -1 { return -1; }
                    black3 += res;
                }
            }
            if black3 == t { 0 } else if black3 > t { -1 } else { black3 }
        }
        for black4 in (1..=n as i32).rev() {
            if black2 % black4 == 0 {
                if dfs(0, 0, black2 / black4, &black1, &nums) == 0 { return black4 - 1; }
            }
        }
        0
    }
}
