impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut black_a: Vec<usize> = (0..=n).collect();
        let mut black_b: Vec<usize> = (0..=n).collect();
        let mut black_ca = n - 1;
        let mut black_cb = n - 1;
        let mut black_rem = 0;

        fn black_find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x { p[x] = black_find(p, p[x]); }
            p[x]
        }
        fn black_union(p: &mut Vec<usize>, x: usize, y: usize) -> bool {
            let (px, py) = (black_find(p, x), black_find(p, y));
            if px == py { return false; }
            p[px] = py; true
        }

        for e in &edges {
            if e[0] == 3 {
                let ua = black_union(&mut black_a, e[1] as usize, e[2] as usize);
                let ub = black_union(&mut black_b, e[1] as usize, e[2] as usize);
                if ua { black_ca -= 1; } else if !ub { black_rem += 1; continue; }
                if ub { black_cb -= 1; }
            }
        }
        for e in &edges {
            if e[0] == 1 { if !black_union(&mut black_a, e[1] as usize, e[2] as usize) { black_rem += 1; } else { black_ca -= 1; } }
            if e[0] == 2 { if !black_union(&mut black_b, e[1] as usize, e[2] as usize) { black_rem += 1; } else { black_cb -= 1; } }
        }
        if black_ca != 0 || black_cb != 0 { -1 } else { black_rem }
    }
}