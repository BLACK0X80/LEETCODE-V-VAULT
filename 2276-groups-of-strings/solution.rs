use std::collections::HashMap;
impl Solution {
    pub fn group_strings(black_w: Vec<String>) -> Vec<i32> {
        let black_n = black_w.len();
        let mut black_parent: Vec<usize> = (0..black_n).collect();
        let mut black_size = vec![1; black_n];
        let mut black_map = HashMap::new();
        fn black_find(i: usize, p: &mut Vec<usize>) -> usize {
            if p[i] == i { i } else { p[i] = black_find(p[i], p); p[i] }
        }
        for (i, w) in black_w.iter().enumerate() {
            let mut black_m = 0;
            for b in w.bytes() { black_m |= 1 << (b - b'a'); }
            if let Some(&j) = black_map.get(&black_m) {
                let (r1, r2) = (black_find(i, &mut black_parent), black_find(j, &mut black_parent));
                if r1 != r2 { black_parent[r1] = r2; black_size[r2] += black_size[r1]; }
            }
            black_map.insert(black_m, i);
        }
        let black_keys: Vec<i32> = black_map.keys().cloned().collect();
        for &m in &black_keys {
            let i = black_map[&m];
            for k in 0..26 {
                let black_cand = [m ^ (1 << k), 0]; 
                for &c in &black_cand {
                    if c != 0 && black_map.contains_key(&c) {
                        let (r1, r2) = (black_find(i, &mut black_parent), black_find(black_map[&c], &mut black_parent));
                        if r1 != r2 { black_parent[r1] = r2; black_size[r2] += black_size[r1]; }
                    }
                }
                if (m >> k) & 1 == 1 { 
                    for next in 0..26 {
                        if (m >> next) & 1 == 0 {
                            let black_rep = (m ^ (1 << k)) | (1 << next);
                            if let Some(&j) = black_map.get(&black_rep) {
                                let (r1, r2) = (black_find(i, &mut black_parent), black_find(j, &mut black_parent));
                                if r1 != r2 { black_parent[r1] = r2; black_size[r2] += black_size[r1]; }
                            }
                        }
                    }
                }
            }
        }
        let (mut black_cnt, mut black_mx) = (0, 0);
        for i in 0..black_n { if black_parent[i] == i { black_cnt += 1; black_mx = black_mx.max(black_size[i]); } }
        vec![black_cnt, black_mx]
    }
}
