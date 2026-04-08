use std::collections::{HashMap, BTreeSet};

impl Solution {
    pub fn subarray_majority(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = nums.len();
        let black_block = (black_n as f64).sqrt() as usize + 1;
        let mut black_qs: Vec<_> = queries.iter().enumerate().collect();
        black_qs.sort_by_key(|&(i, q)| (q[0] as usize / black_block, q[1]));

        let mut black_f = HashMap::new();
        let mut black_fn: HashMap<i32, BTreeSet<i32>> = HashMap::new();
        let mut black_mf = 0;
        let mut black_ans = vec![-1; queries.len()];
        let (mut black_cur_l, mut black_cur_r) = (0, 0);
        Self::black_add(nums[0], &mut black_f, &mut black_fn, &mut black_mf);

        for (black_idx, black_q) in black_qs {
            let (ql, qr, qt) = (black_q[0] as usize, black_q[1] as usize, black_q[2]);
            
            while black_cur_r < qr {
                black_cur_r += 1;
                Self::black_add(nums[black_cur_r], &mut black_f, &mut black_fn, &mut black_mf);
            }
            while black_cur_l > ql {
                black_cur_l -= 1;
                Self::black_add(nums[black_cur_l], &mut black_f, &mut black_fn, &mut black_mf);
            }
            while black_cur_r > qr {
                Self::black_rem(nums[black_cur_r], &mut black_f, &mut black_fn, &mut black_mf);
                black_cur_r -= 1;
            }
            while black_cur_l < ql {
                Self::black_rem(nums[black_cur_l], &mut black_f, &mut black_fn, &mut black_mf);
                black_cur_l += 1;
            }

            let mut res = -1;
            for f in (qt..=black_mf).rev() {
                if let Some(s) = black_fn.get(&f) {
                    if let Some(&v) = s.iter().next() {
                        res = v;
                        break;
                    }
                }
            }
            black_ans[black_idx] = res;
        }
        black_ans
    }

    fn black_add(x: i32, f: &mut HashMap<i32, i32>, fn_map: &mut HashMap<i32, BTreeSet<i32>>, mf: &mut i32) {
        let old_f = f.get(&x).cloned().unwrap_or(0);
        if old_f > 0 { fn_map.get_mut(&old_f).unwrap().remove(&x); }
        let new_f = old_f + 1;
        f.insert(x, new_f);
        fn_map.entry(new_f).or_default().insert(x);
        if new_f > *mf { *mf = new_f; }
    }

    fn black_rem(x: i32, f: &mut HashMap<i32, i32>, fn_map: &mut HashMap<i32, BTreeSet<i32>>, mf: &mut i32) {
        let old_f = *f.get(&x).unwrap();
        fn_map.get_mut(&old_f).unwrap().remove(&x);
        let new_f = old_f - 1;
        if new_f > 0 {
            f.insert(x, new_f);
            fn_map.entry(new_f).or_default().insert(x);
        } else { f.remove(&x); }
        if old_f == *mf && fn_map.get(&old_f).map_or(true, |s| s.is_empty()) { *mf -= 1; }
    }
}
