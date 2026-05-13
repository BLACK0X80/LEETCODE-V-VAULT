use std::collections::HashSet;

impl Solution {
    pub fn is_possible(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut adj = vec![HashSet::new(); n as usize + 1];
        let mut deg = vec![0; n as usize + 1];
        for e in &edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            adj[u].insert(v);
            adj[v].insert(u);
            deg[u] += 1;
            deg[v] += 1;
        }
        let odds: Vec<usize> = (1..=n as usize).filter(|&i| deg[i] & 1 == 1).collect();
        match odds.len() {
            0 => true,
            2 => {
                let a = odds[0];
                let b = odds[1];
                if !adj[a].contains(&b) { return true; }
                for i in 1..=n as usize {
                    if i != a && i != b && !adj[a].contains(&i) && !adj[b].contains(&i) { return true; }
                }
                false
            }
            4 => {
                let a = odds[0]; let b = odds[1]; let c = odds[2]; let d = odds[3];
                (!adj[a].contains(&b) && !adj[c].contains(&d)) ||
                (!adj[a].contains(&c) && !adj[b].contains(&d)) ||
                (!adj[a].contains(&d) && !adj[b].contains(&c))
            }
            _ => false
        }
    }
}