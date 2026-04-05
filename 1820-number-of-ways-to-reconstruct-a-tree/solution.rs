use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();
        for p in &pairs {
            adj.entry(p[0]).or_default().insert(p[1]);
            adj.entry(p[1]).or_default().insert(p[0]);
        }

        let mut nodes: Vec<i32> = adj.keys().copied().collect();
        let n = nodes.len();
        nodes.sort_by_key(|&x| adj[&x].len());

        let mut multi = false;

        for &node in &nodes {
            let deg = adj[&node].len();
            let mut par = -1i32;
            let mut par_deg = usize::MAX;

            for &nb in &adj[&node] {
                let d = adj[&nb].len();
                if d >= deg && d < par_deg {
                    par = nb;
                    par_deg = d;
                }
            }

            if par == -1 {
                if deg != n - 1 { return 0; }
                continue;
            }

            if par_deg == deg { multi = true; }

            for &nb in adj[&node].iter() {
                if nb == par { continue; }
                if !adj[&par].contains(&nb) { return 0; }
            }
        }

        let root_count = nodes.iter().filter(|&&x| adj[&x].len() == n - 1).count();
        if root_count == 0 { return 0; }

        if multi { 2 } else { 1 }
    }
}
