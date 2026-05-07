impl Solution {
    pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let black_n = nums.len();
        let black_mod = 1_000_000_007i64;
        let mut black_tree = vec![[0i64; 4]; 4 * black_n];

        fn black_merge(l: [i64; 4], r: [i64; 4]) -> [i64; 4] {
            let mut res = [0i64; 4];
            res[0] = (l[0] + r[2]).max(l[1] + r[0]).max(l[0] + r[0]);
            res[1] = (l[0] + r[3]).max(l[1] + r[1]).max(l[0] + r[1]);
            res[2] = (l[2] + r[2]).max(l[3] + r[0]).max(l[2] + r[0]);
            res[3] = (l[2] + r[3]).max(l[3] + r[1]).max(l[2] + r[1]);
            res
        }

        fn black_build(idx: usize, s: usize, e: usize, nums: &Vec<i32>, tree: &mut Vec<[i64; 4]>) {
            if s == e {
                tree[idx] = [0, -1_000_000_000, -1_000_000_000, (nums[s] as i64).max(0)];
                return;
            }
            let mid = (s + e) / 2;
            black_build(idx * 2, s, mid, nums, tree);
            black_build(idx * 2 + 1, mid + 1, e, nums, tree);
            tree[idx] = black_merge(tree[idx * 2], tree[idx * 2 + 1]);
        }

        fn black_update(idx: usize, s: usize, e: usize, pos: usize, val: i32, tree: &mut Vec<[i64; 4]>) {
            if s == e {
                tree[idx][3] = (val as i64).max(0);
                return;
            }
            let mid = (s + e) / 2;
            if pos <= mid { black_update(idx * 2, s, mid, pos, val, tree); }
            else { black_update(idx * 2 + 1, mid + 1, e, pos, val, tree); }
            tree[idx] = black_merge(tree[idx * 2], tree[idx * 2 + 1]);
        }

        black_build(1, 0, black_n - 1, &nums, &mut black_tree);
        let mut black_ans = 0i64;
        for q in queries {
            black_update(1, 0, black_n - 1, q[0] as usize, q[1], &mut black_tree);
            let black_max_val = black_tree[1][0].max(black_tree[1][1]).max(black_tree[1][2]).max(black_tree[1][3]);
            black_ans = (black_ans + black_max_val) % black_mod;
        }
        black_ans as i32
    }
}