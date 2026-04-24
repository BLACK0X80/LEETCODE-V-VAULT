impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let inf = i32::MAX / 2;
        let mut dist = vec![vec![inf; n]; n];
        for i in 0..n { dist[i][i] = 0; }
        for e in &edges {
            dist[e[0] as usize][e[1] as usize] = e[2];
            dist[e[1] as usize][e[0] as usize] = e[2];
        }
        for k in 0..n { for i in 0..n { for j in 0..n {
            if dist[i][k] + dist[k][j] < dist[i][j] { dist[i][j] = dist[i][k] + dist[k][j]; }
        }}}
        let (mut ans, mut min_cnt) = (0, n + 1);
        for i in 0..n {
            let cnt = dist[i].iter().filter(|&&d| d <= distance_threshold).count() - 1;
            if cnt <= min_cnt { min_cnt = cnt; ans = i; }
        }
        ans as i32
    }
}