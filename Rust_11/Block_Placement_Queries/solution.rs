impl Solution {
    pub fn get_results(black_qs: Vec<Vec<i32>>) -> Vec<bool> {
        let (mut black_b, mut black_s) = (vec![0; 50005], std::collections::BTreeSet::from([0, 50001]));
        for black_q in &black_qs { if black_q[0] == 1 { black_s.insert(black_q[1]); } }
        let (black_o, mut black_u) = (black_s.iter().cloned().collect::<Vec<_>>(), |mut black_i: usize, black_v: i32, black_t: &mut Vec<i32>| { while black_i < 50005 { black_t[black_i] = black_t[black_i].max(black_v); black_i += black_i & black_i.wrapping_neg(); } });
        for black_i in 1..black_o.len() { black_u(black_o[black_i] as usize, black_o[black_i] - black_o[black_i-1], &mut black_b); }
        black_qs.into_iter().rev().filter_map(|black_q| if black_q[0] == 1 {
            black_s.remove(&black_q[1]); let black_p = *black_s.range(..black_q[1]).next_back().unwrap();
            if let Some(&black_n) = black_s.range(black_q[1]..).next() { black_u(black_n as usize, black_n - black_p, &mut black_b); } None
        } else {
            let (black_x, mut black_i, mut black_m) = (black_q[1], black_q[1] as usize, 0);
            while black_i > 0 { black_m = black_m.max(black_b[black_i]); black_i &= black_i - 1; }
            Some(black_m.max(black_x - *black_s.range(..=black_x).next_back().unwrap()) >= black_q[2])
        }).collect::<Vec<_>>().into_iter().rev().collect()
    }
}