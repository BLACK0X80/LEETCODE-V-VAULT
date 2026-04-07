use std::collections::HashMap;

impl Solution {
    pub fn minimum_cost(source: String, target: String, original: Vec<String>, changed: Vec<String>, cost: Vec<i32>) -> i64 {
        let src = source.as_bytes();
        let tgt = target.as_bytes();
        let n = src.len();
        
        let mut by_len: HashMap<usize, HashMap<(String, String), i64>> = HashMap::new();
        for ((o, c), &w) in original.iter().zip(changed.iter()).zip(cost.iter()) {
            let len = o.len();
            let key = (o.clone(), c.clone());
            by_len.entry(len).or_default().entry(key).and_modify(|e| *e = (*e).min(w as i64)).or_insert(w as i64);
        }
        
        let mut all_costs: HashMap<usize, HashMap<(String, String), i64>> = HashMap::new();
        
        for (len, graph) in by_len {
            let mut nodes: Vec<String> = Vec::new();
            let mut node_idx: HashMap<String, usize> = HashMap::new();
            
            for (o, c) in graph.keys() {
                if !node_idx.contains_key(o) {
                    node_idx.insert(o.clone(), nodes.len());
                    nodes.push(o.clone());
                }
                if !node_idx.contains_key(c) {
                    node_idx.insert(c.clone(), nodes.len());
                    nodes.push(c.clone());
                }
            }
            
            let sz = nodes.len();
            let mut dist = vec![vec![i64::MAX / 4; sz]; sz];
            for i in 0..sz { dist[i][i] = 0; }
            
            for ((o, c), &w) in &graph {
                let u = node_idx[o];
                let v = node_idx[c];
                dist[u][v] = dist[u][v].min(w);
            }
            
            for k in 0..sz {
                for i in 0..sz {
                    if dist[i][k] == i64::MAX / 4 { continue; }
                    for j in 0..sz {
                        if dist[k][j] == i64::MAX / 4 { continue; }
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
            
            let mut result: HashMap<(String, String), i64> = HashMap::new();
            for i in 0..sz {
                for j in 0..sz {
                    if dist[i][j] < i64::MAX / 4 {
                        result.insert((nodes[i].clone(), nodes[j].clone()), dist[i][j]);
                    }
                }
            }
            all_costs.insert(len, result);
        }
        
        let mut dp = vec![i64::MAX / 4; n + 1];
        dp[0] = 0;
        
        for i in 0..n {
            if dp[i] == i64::MAX / 4 { continue; }
            
            if src[i] == tgt[i] {
                dp[i + 1] = dp[i + 1].min(dp[i]);
            }
            
            for (&len, costs) in &all_costs {
                if i + len > n { continue; }
                
                let s = std::str::from_utf8(&src[i..i+len]).unwrap();
                let t = std::str::from_utf8(&tgt[i..i+len]).unwrap();
                
                if let Some(&c) = costs.get(&(s.to_string(), t.to_string())) {
                    dp[i + len] = dp[i + len].min(dp[i] + c);
                }
            }
        }
        
        if dp[n] == i64::MAX / 4 { -1 } else { dp[n] }
    }
}
