use std::collections::{VecDeque, HashMap};

impl Solution {
    pub fn minimum_cost(black: String, blacks: Vec<String>, bl: Vec<i32>) -> i32 {
        let (b, bn) = (black.as_bytes(), black.len());
        const INF: i32 = i32::MAX / 2;
        let mut go: Vec<HashMap<u8, usize>> = vec![HashMap::new(), HashMap::new()];
        let mut sfx = vec![0usize; 2];
        let mut dict = vec![0usize; 2];
        let mut cost = vec![INF; 2];
        let mut blen = vec![-1i32, 0i32];

        for (w, &c) in blacks.iter().zip(bl.iter()) {
            let mut u = 1usize;
            for &byte in w.as_bytes() {
                let x = byte - b'a';
                if !go[u].contains_key(&x) {
                    let pl = blen[u];
                    go.push(HashMap::new());
                    sfx.push(0); dict.push(0); cost.push(INF);
                    blen.push(pl + 1);
                    let nn = go.len() - 1;
                    go[u].insert(x, nn);
                }
                u = go[u][&x];
            }
            cost[u] = cost[u].min(c);
        }

        let sz = go.len();
        let mut go2 = vec![[0usize; 26]; sz];
        for i in 0..sz { for (&k, &v) in &go[i] { go2[i][k as usize] = v; } }
        drop(go);

        go2[0].iter_mut().for_each(|x| if *x == 0 { *x = 1; });

        let mut q = VecDeque::from([1usize]);
        while let Some(u) = q.pop_front() {
            for x in 0..26 {
                let v = go2[u][x];
                let s = go2[sfx[u]][x];
                if v == 0 { go2[u][x] = s; }
                else {
                    sfx[v] = s;
                    dict[v] = if cost[s] < INF { s } else { dict[s] };
                    q.push_back(v);
                }
            }
        }

        let mut dp = vec![INF; bn + 1];
        dp[0] = 0;
        let mut u = 1usize;
        for i in 1..=bn {
            u = go2[u][(b[i-1] - b'a') as usize];
            let mut v = if cost[u] < INF { u } else { dict[u] };
            while v != 0 {
                let pl = blen[v] as usize;
                if i >= pl && dp[i - pl] < INF {
                    dp[i] = dp[i].min(dp[i - pl] + cost[v]);
                }
                v = dict[v];
            }
        }

        if dp[bn] >= INF { -1 } else { dp[bn] }
    }
}
