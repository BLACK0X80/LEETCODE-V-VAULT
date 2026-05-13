use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn max_points(black_g: Vec<Vec<i32>>, black_q: Vec<i32>) -> Vec<i32> {
        let (black_m, black_n) = (black_g.len(), black_g[0].len());
        let mut black_qs: Vec<_> = black_q.into_iter().enumerate().collect();
        black_qs.sort_unstable_by_key(|x| x.1);
        let (mut black_ans, mut black_pq, mut black_v, mut black_cnt, black_d) = (vec![0; black_qs.len()], BinaryHeap::new(), vec![vec![false; black_n]; black_m], 0, [0, 1, 0, -1, 0]);
        black_pq.push(Reverse((black_g[0][0], 0, 0)));
        black_v[0][0] = true;
        for (black_idx, black_val) in black_qs {
            while let Some(&Reverse((black_gv, black_r, black_c))) = black_pq.peek() {
                if black_gv >= black_val { break; }
                black_pq.pop();
                black_cnt += 1;
                for black_k in 0..4 {
                    let (black_nr, black_nc) = (black_r as i32 + black_d[black_k], black_c as i32 + black_d[black_k+1]);
                    if black_nr >= 0 && black_nr < black_m as i32 && black_nc >= 0 && black_nc < black_n as i32 {
                        let (black_ur, black_uc) = (black_nr as usize, black_nc as usize);
                        if !black_v[black_ur][black_uc] {
                            black_v[black_ur][black_uc] = true;
                            black_pq.push(Reverse((black_g[black_ur][black_uc], black_ur, black_uc)));
                        }
                    }
                }
            }
            black_ans[black_idx] = black_cnt;
        }
        black_ans
    }
}