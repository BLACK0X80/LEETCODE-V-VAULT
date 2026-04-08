impl Solution {
    pub fn maximum_strong_pair_xor(mut black_nums: Vec<i32>) -> i32 {
        black_nums.sort_unstable();
        let mut black_t = vec![[0, 0, 0]];
        let (mut black_l, mut black_ans) = (0, 0);

        for &black_r_val in &black_nums {
            Self::black_upd(&mut black_t, black_r_val, 1);
            while black_nums[black_l] * 2 < black_r_val {
                Self::black_upd(&mut black_t, black_nums[black_l], -1);
                black_l += 1;
            }
            black_ans = black_ans.max(Self::black_get(&black_t, black_r_val));
        }
        black_ans
    }

    fn black_upd(t: &mut Vec<[i32; 3]>, v: i32, d: i32) {
        let mut curr = 0;
        for i in (0..20).rev() {
            let b = ((v >> i) & 1) as usize;
            if t[curr][b] == 0 {
                t[curr][b] = t.len() as i32;
                t.push([0, 0, 0]);
            }
            curr = t[curr][b] as usize;
            t[curr][2] += d;
        }
    }

    fn black_get(t: &Vec<[i32; 3]>, v: i32) -> i32 {
        let (mut curr, mut res) = (0, 0);
        for i in (0..20).rev() {
            let b = ((v >> i) & 1) as usize;
            let target = 1 - b;
            let next = t[curr][target] as usize;
            if next != 0 && t[next][2] > 0 {
                res |= 1 << i;
                curr = next;
            } else {
                curr = t[curr][b] as usize;
            }
        }
        res
    }
}
