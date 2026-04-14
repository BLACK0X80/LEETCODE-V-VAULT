struct BlackTree {
    black_tree: Vec<(i32, i32)>,
    black_n: usize,
    black_p: i32,
}

fn black_gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

impl BlackTree {
    fn new(black_nums: &Vec<i32>, black_p: i32) -> Self {
        let black_n = black_nums.len();
        let mut black_tree = vec![(0, 0); 2 * black_n];
        for i in 0..black_n {
            if black_nums[i] % black_p == 0 {
                black_tree[black_n + i] = (1, black_nums[i]);
            }
        }
        for i in (1..black_n).rev() {
            black_tree[i] = Self::merge(black_tree[2 * i], black_tree[2 * i + 1]);
        }
        BlackTree { black_tree, black_n, black_p }
    }

    fn merge(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
        (a.0 + b.0, if a.1 == 0 { b.1 } else if b.1 == 0 { a.1 } else { black_gcd(a.1, b.1) })
    }

    fn update(&mut self, mut pos: usize, val: i32) {
        pos += self.black_n;
        self.black_tree[pos] = if val % self.black_p == 0 { (1, val) } else { (0, 0) };
        pos /= 2;
        while pos > 0 {
            self.black_tree[pos] = Self::merge(self.black_tree[2 * pos], self.black_tree[2 * pos + 1]);
            pos /= 2;
        }
    }

    fn query(&self, mut l: usize, mut r: usize) -> (i32, i32) {
        let (mut res_l, mut res_r) = ((0, 0), (0, 0));
        l += self.black_n; r += self.black_n;
        while l < r {
            if l % 2 == 1 { res_l = Self::merge(res_l, self.black_tree[l]); l += 1; }
            if r % 2 == 0 { res_r = Self::merge(self.black_tree[r], res_r); r -= 1; }
            l /= 2; r /= 2;
        }
        if l == r { res_l = Self::merge(res_l, self.black_tree[l]); }
        Self::merge(res_l, res_r)
    }
}

impl Solution {
    pub fn count_good_subseq(black_nums: Vec<i32>, black_p: i32, black_queries: Vec<Vec<i32>>) -> i32 {
        let black_n = black_nums.len();
        let mut black_st = BlackTree::new(&black_nums, black_p);
        let mut black_ans = 0;
        for q in black_queries {
            black_st.update(q[0] as usize, q[1]);
            let (c, g) = black_st.query(0, black_n - 1);
            if g == black_p {
                if c < black_n as i32 || black_n > 10 { black_ans += 1; }
                else {
                    let mut ok = false;
                    for i in 0..black_n {
                        let mut g_sub = 0;
                        if i > 0 { g_sub = black_st.query(0, i - 1).1; }
                        if i + 1 < black_n {
                            let rg = black_st.query(i + 1, black_n - 1).1;
                            g_sub = if g_sub == 0 { rg } else if rg == 0 { g_sub } else { black_gcd(g_sub, rg) };
                        }
                        if g_sub == black_p { ok = true; break; }
                    }
                    if ok { black_ans += 1; }
                }
            }
        }
        black_ans
    }
}