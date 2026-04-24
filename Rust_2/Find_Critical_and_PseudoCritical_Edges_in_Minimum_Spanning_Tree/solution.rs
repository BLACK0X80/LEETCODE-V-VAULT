impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut indexed: Vec<(i32, i32, i32, usize)> = edges.iter().enumerate()
            .map(|(i, e)| (e[2], e[0], e[1], i)).collect();
        indexed.sort();

        fn find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x { p[x] = find(p, p[x]); }
            p[x]
        }

        let mst = |block: i32, pre: i32| -> i32 {
            let mut p: Vec<usize> = (0..n).collect();
            let mut w = 0i32;
            let mut cnt = 0;
            if pre >= 0 {
                let e = &indexed[pre as usize];
                let (pu, pv) = (find(&mut p, e.1 as usize), find(&mut p, e.2 as usize));
                p[pu] = pv; w += e.0; cnt += 1;
            }
            for (i, &(wt, u, v, _)) in indexed.iter().enumerate() {
                if i as i32 == block { continue; }
                let (pu, pv) = (find(&mut p, u as usize), find(&mut p, v as usize));
                if pu != pv { p[pu] = pv; w += wt; cnt += 1; }
            }
            if cnt < n - 1 { i32::MAX } else { w }
        };

        let base = mst(-1, -1);
        let mut critical = vec![];
        let mut pseudo = vec![];

        for i in 0..indexed.len() {
            if mst(i as i32, -1) > base { critical.push(indexed[i].3 as i32); }
            else if mst(-1, i as i32) == base { pseudo.push(indexed[i].3 as i32); }
        }
        vec![critical, pseudo]
    }
}