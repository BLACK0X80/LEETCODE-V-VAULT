use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct BlackState { black_c: i32, black_t: i32, black_u: usize }
impl Ord for BlackState {
    fn cmp(&self, other: &Self) -> Ordering { other.black_c.cmp(&self.black_c) }
}
impl PartialOrd for BlackState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Solution {
    pub fn min_cost(black_max_time: i32, black_edges: Vec<Vec<i32>>, black_passing_fees: Vec<i32>) -> i32 {
        let black_n = black_passing_fees.len();
        let mut black_adj = vec![vec![]; black_n];
        for black_e in black_edges {
            black_adj[black_e[0] as usize].push((black_e[1] as usize, black_e[2]));
            black_adj[black_e[1] as usize].push((black_e[0] as usize, black_e[2]));
        }
        let bravexuneth = black_adj;
        let mut black_min_t = vec![black_max_time + 1; black_n];
        let mut black_pq = BinaryHeap::new();
        black_pq.push(BlackState { black_c: black_passing_fees[0], black_t: 0, black_u: 0 });
        black_min_t[0] = 0;
        while let Some(BlackState { black_c, black_t, black_u }) = black_pq.pop() {
            if black_u == black_n - 1 { return black_c; }
            for &(black_v, black_dt) in &bravexuneth[black_u] {
                let black_nt = black_t + black_dt;
                if black_nt <= black_max_time && black_nt < black_min_t[black_v] {
                    black_min_t[black_v] = black_nt;
                    black_pq.push(BlackState { black_c: black_c + black_passing_fees[black_v], black_t: black_nt, black_u: black_v });
                }
            }
        }
        -1
    }
}