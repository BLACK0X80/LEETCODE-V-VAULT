impl Solution {
    pub fn distribute_cookies(black_c: Vec<i32>, black_k: i32) -> i32 {
        let mut black_res = i32::MAX; let mut black_dist = vec![0; black_k as usize];
        fn black_bt(black_i: usize, black_v: &Vec<i32>, black_d: &mut Vec<i32>, black_ans: &mut i32) { if black_i == black_v.len() { *black_ans = (*black_ans).min(*black_d.iter().max().unwrap()); return; } for black_j in 0..black_d.len() { if black_d[black_j] + black_v[black_i] < *black_ans { black_d[black_j] += black_v[black_i]; black_bt(black_i + 1, black_v, black_d, black_ans); black_d[black_j] -= black_v[black_i]; } if black_d[black_j] == 0 { break; } } }
        black_bt(0, &black_c, &mut black_dist, &mut black_res); black_res
    }
}