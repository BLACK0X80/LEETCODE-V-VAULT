impl Solution {
    pub fn longest_balanced(black_nums: Vec<i32>) -> i32 {
        let black_n = black_nums.len();
        if black_n == 0 { return 0; }

        let mut black_next = vec![black_n; black_n];
        let mut black_last = std::collections::HashMap::new();
        for i in (0..black_n).rev() {
            if let Some(&p) = black_last.get(&black_nums[i]) { black_next[i] = p; }
            black_last.insert(black_nums[i], i);
        }

        let mut black_bal = vec![0i32; black_n];
        let (mut black_e, mut black_o, mut black_b) = (std::collections::HashSet::new(), std::collections::HashSet::new(), 0);
        for i in 0..black_n {
            let x = black_nums[i];
            if x % 2 == 0 { if black_e.insert(x) { black_b += 1; } }
            else { if black_o.insert(x) { black_b -= 1; } }
            black_bal[i] = black_b;
        }

        let mut black_tree = BlackSegTree::new(&black_bal);
        let mut black_max = 0;

        for l in 0..black_n {
            if l + black_max as usize >= black_n { break; }
            let black_res = black_tree.find_last(1, 0, black_n - 1, l + black_max as usize);
            if black_res != -1 { black_max = black_max.max(black_res - l as i32 + 1); }

            if l < black_n - 1 {
                let end = black_next[l] - 1;
                if end >= l + 1 {
                    let delta = if black_nums[l] % 2 == 0 { -1 } else { 1 };
                    black_tree.update(1, 0, black_n - 1, l + 1, end, delta);
                }
            }
        }
        black_max
    }
}

struct BlackSegTree {
    min: Vec<i32>,
    max: Vec<i32>,
    lazy: Vec<i32>,
}

impl BlackSegTree {
    fn new(data: &[i32]) -> Self {
        let n = data.len();
        let mut tree = Self { min: vec![0; 4 * n], max: vec![0; 4 * n], lazy: vec![0; 4 * n] };
        tree.build(data, 1, 0, n - 1);
        tree
    }

    fn build(&mut self, data: &[i32], node: usize, s: usize, e: usize) {
        if s == e { self.min[node] = data[s]; self.max[node] = data[s]; }
        else {
            let m = (s + e) / 2;
            self.build(data, 2 * node, s, m);
            self.build(data, 2 * node + 1, m + 1, e);
            self.min[node] = self.min[2 * node].min(self.min[2 * node + 1]);
            self.max[node] = self.max[2 * node].max(self.max[2 * node + 1]);
        }
    }

    fn push(&mut self, n: usize) {
        let lz = self.lazy[n];
        if lz != 0 {
            self.min[2 * n] += lz; self.max[2 * n] += lz; self.lazy[2 * n] += lz;
            self.min[2 * n + 1] += lz; self.max[2 * n + 1] += lz; self.lazy[2 * n + 1] += lz;
            self.lazy[n] = 0;
        }
    }

    fn update(&mut self, n: usize, s: usize, e: usize, l: usize, r: usize, v: i32) {
        if l <= s && e <= r { self.min[n] += v; self.max[n] += v; self.lazy[n] += v; }
        else {
            self.push(n);
            let m = (s + e) / 2;
            if l <= m { self.update(2 * n, s, m, l, r, v); }
            if r > m { self.update(2 * n + 1, m + 1, e, l, r, v); }
            self.min[n] = self.min[2 * n].min(self.min[2 * n + 1]);
            self.max[n] = self.max[2 * n].max(self.max[2 * n + 1]);
        }
    }

    fn find_last(&mut self, n: usize, s: usize, e: usize, min_idx: usize) -> i32 {
        if e < min_idx || self.min[n] > 0 || self.max[n] < 0 { return -1; }
        if s == e { return s as i32; }
        self.push(n);
        let m = (s + e) / 2;
        let r = self.find_last(2 * n + 1, m + 1, e, min_idx);
        if r != -1 { r } else { self.find_last(2 * n, s, m, min_idx) }
    }
}
