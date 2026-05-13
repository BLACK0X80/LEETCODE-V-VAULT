impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut parent: Vec<usize> = (0..n).collect();
        fn find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x { p[x] = find(p, p[x]); } p[x]
        }
        for e in &edges {
            let (a, b) = (find(&mut parent, e[0] as usize), find(&mut parent, e[1] as usize));
            if a != b { parent[a] = b; }
        }
        let mut nodes = vec![0usize; n];
        let mut edge_cnt = vec![0usize; n];
        for i in 0..n { nodes[find(&mut parent, i)] += 1; }
        for e in &edges { edge_cnt[find(&mut parent, e[0] as usize)] += 1; }
        (0..n).filter(|&i| find(&mut parent, i) == i)
            .filter(|&i| edge_cnt[i] == nodes[i] * (nodes[i] - 1) / 2)
            .count() as i32
    }
}