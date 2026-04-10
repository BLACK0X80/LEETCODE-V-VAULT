impl Solution {
    pub fn palindrome_path(
        black_n: i32,
        black_edges: Vec<Vec<i32>>,
        mut black_s: String,
        black_queries: Vec<String>,
    ) -> Vec<bool> {
        let black_n = black_n as usize;
        let mut black_g = vec![Vec::new(); black_n];
        for black_e in black_edges {
            black_g[black_e[0] as usize].push(black_e[1] as usize);
            black_g[black_e[1] as usize].push(black_e[0] as usize);
        }

        let black_log = 17;
        let mut black_parent = vec![vec![None; black_n]; black_log];
        let mut black_depth = vec![0; black_n];
        let mut black_tin = vec![0; black_n];
        let mut black_tout = vec![0; black_n];
        let mut black_timer = 0;

        Self::dfs(
            0,
            None,
            &black_g,
            &mut black_parent,
            &mut black_depth,
            &mut black_tin,
            &mut black_tout,
            &mut black_timer,
        );

        for black_k in 1..black_log {
            for black_i in 0..black_n {
                if let Some(black_p) = black_parent[black_k - 1][black_i] {
                    black_parent[black_k][black_i] = black_parent[black_k - 1][black_p];
                }
            }
        }

        let mut black_fenwick = Fenwick::new(black_n);
        let mut black_s_bytes = black_s.into_bytes();

        let black_get_mask = |black_c: u8| 1 << (black_c - b'a');

        for black_i in 0..black_n {
            let black_m = black_get_mask(black_s_bytes[black_i]);
            black_fenwick.add(black_tin[black_i], black_m);
            black_fenwick.add(black_tout[black_i] + 1, black_m);
        }

        let mut black_ans = Vec::with_capacity(black_queries.len());

        for black_q in black_queries {
            let black_parts: Vec<&str> = black_q.split_whitespace().collect();
            if black_parts[0] == "update" {
                let black_u = black_parts[1].parse::<usize>().unwrap();
                let black_new_c = black_parts[2].as_bytes()[0];
                let black_delta = black_get_mask(black_s_bytes[black_u]) ^ black_get_mask(black_new_c);
                black_s_bytes[black_u] = black_new_c;
                black_fenwick.add(black_tin[black_u], black_delta);
                black_fenwick.add(black_tout[black_u] + 1, black_delta);
            } else {
                let black_u = black_parts[1].parse::<usize>().unwrap();
                let black_v = black_parts[2].parse::<usize>().unwrap();
                let black_w = Self::lca(black_u, black_v, &black_parent, &black_depth, black_log);
                let black_m = black_fenwick.sum(black_tin[black_u])
                    ^ black_fenwick.sum(black_tin[black_v])
                    ^ black_get_mask(black_s_bytes[black_w]);

                black_ans.push(black_m == 0 || (black_m & (black_m - 1)) == 0);
            }
        }
        black_ans
    }

    fn dfs(
        black_u: usize,
        black_p: Option<usize>,
        black_g: &Vec<Vec<usize>>,
        black_parent: &mut Vec<Vec<Option<usize>>>,
        black_depth: &mut Vec<usize>,
        black_tin: &mut Vec<usize>,
        black_tout: &mut Vec<usize>,
        black_timer: &mut usize,
    ) {
        black_tin[black_u] = *black_timer;
        *black_timer += 1;
        black_parent[0][black_u] = black_p;
        for &black_v in &black_g[black_u] {
            if Some(black_v) == black_p { continue; }
            black_depth[black_v] = black_depth[black_u] + 1;
            Self::dfs(black_v, Some(black_u), black_g, black_parent, black_depth, black_tin, black_tout, black_timer);
        }
        black_tout[black_u] = *black_timer - 1;
    }

    fn lca(
        mut black_u: usize,
        mut black_v: usize,
        black_parent: &Vec<Vec<Option<usize>>>,
        black_depth: &Vec<usize>,
        black_log: usize,
    ) -> usize {
        if black_depth[black_u] < black_depth[black_v] { std::mem::swap(&mut black_u, &mut black_v); }
        let black_diff = black_depth[black_u] - black_depth[black_v];
        for black_k in 0..black_log {
            if (black_diff >> black_k) & 1 == 1 {
                black_u = black_parent[black_k][black_u].unwrap();
            }
        }
        if black_u == black_v { return black_u; }
        for black_k in (0..black_log).rev() {
            if black_parent[black_k][black_u] != black_parent[black_k][black_v] {
                black_u = black_parent[black_k][black_u].unwrap();
                black_v = black_parent[black_k][black_v].unwrap();
            }
        }
        black_parent[0][black_u].unwrap()
    }
}

struct Fenwick {
    black_n: usize,
    black_bit: Vec<i32>,
}

impl Fenwick {
    fn new(black_n: usize) -> Self {
        Self { black_n, black_bit: vec![0; black_n + 1] }
    }
    fn add(&mut self, mut black_i: usize, black_val: i32) {
        black_i += 1;
        while black_i <= self.black_n {
            self.black_bit[black_i] ^= black_val;
            black_i += (black_i as isize & -(black_i as isize)) as usize;
        }
    }
    fn sum(&self, mut black_i: usize) -> i32 {
        black_i += 1;
        let mut black_res = 0;
        while black_i > 0 {
            black_res ^= self.black_bit[black_i];
            black_i -= (black_i as isize & -(black_i as isize)) as usize;
        }
        black_res
    }
}
