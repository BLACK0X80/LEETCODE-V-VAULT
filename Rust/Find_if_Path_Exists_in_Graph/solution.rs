impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut parent: Vec<usize> = (0..n as usize).collect();
        fn find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x { p[x] = find(p, p[x]); } p[x]
        }
        for e in &edges {
            let (a, b) = (find(&mut parent, e[0] as usize), find(&mut parent, e[1] as usize));
            if a != b { parent[a] = b; }
        }
        find(&mut parent, source as usize) == find(&mut parent, destination as usize)
    }
}