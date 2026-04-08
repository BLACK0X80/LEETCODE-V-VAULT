use std::collections::{BTreeMap, VecDeque, HashSet};
impl Solution {
    pub fn find_all_people(black_n: i32, black_m: Vec<Vec<i32>>, black_f: i32) -> Vec<i32> {
        let mut black_map: BTreeMap<i32, Vec<(i32, i32)>> = BTreeMap::new();
        for black_v in black_m { black_map.entry(black_v[2]).or_default().push((black_v[0], black_v[1])); }
        let mut black_has = vec![false; black_n as usize];
        black_has[0] = true; black_has[black_f as usize] = true;
        for (_, black_list) in black_map {
            let (mut black_adj, mut black_q, mut black_set) = (std::collections::HashMap::new(), VecDeque::new(), HashSet::new());
            for (black_u, black_v) in black_list {
                black_adj.entry(black_u).or_insert(vec![]).push(black_v);
                black_adj.entry(black_v).or_insert(vec![]).push(black_u);
                if black_has[black_u as usize] && black_set.insert(black_u) { black_q.push_back(black_u); }
                if black_has[black_v as usize] && black_set.insert(black_v) { black_q.push_back(black_v); }
            }
            while let Some(black_curr) = black_q.pop_front() {
                if let Some(black_neighs) = black_adj.get(&black_curr) {
                    for &black_next in black_neighs {
                        if !black_has[black_next as usize] { black_has[black_next as usize] = true; black_q.push_back(black_next); }
                    }
                }
            }
        }
        black_has.into_iter().enumerate().filter(|&(_, black_b)| black_b).map(|(black_i, _)| black_i as i32).collect()
    }
}
