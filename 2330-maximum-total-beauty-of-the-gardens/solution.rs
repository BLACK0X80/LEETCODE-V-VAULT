impl Solution {
    pub fn maximum_beauty(mut black: Vec<i32>, mut bl: i64, black0: i32, black1: i32, black2: i32) -> i64 {
        let (t, f, p) = (black0 as i64, black1 as i64, black2 as i64);
        black.sort_by(|a, b| b.cmp(a));
        let (mut b1, mut blk, mut res) = (0i64, 0i64, 0i64);
        let sz = black.len() as i64;

        while b1 < sz {
            if t - black[b1 as usize] as i64 > bl { break; }
            bl -= (t - black[b1 as usize] as i64).max(0);
            b1 += 1;
        }

        if b1 == sz {
            let last = (black.last().copied().unwrap_or(0)) as i64;
            return (sz * f).max((sz-1)*f + if last < t { (t-1)*p } else { f });
        }

        let mut b2 = sz - 1;
        let mut min_f = black[b2 as usize] as i64;

        while min_f < t {
            while b2 >= b1 && (black[b2 as usize] as i64) <= min_f {
                blk += black[b2 as usize] as i64;
                b2 -= 1;
            }
            let needed = (sz - b2 - 1) * min_f - blk;
            if needed > bl {
                b1 -= 1;
                if b1 < 0 { break; }
                bl += (t - black[b1 as usize] as i64).max(0);
            } else {
                res = res.max(b1 * f + min_f * p);
                min_f += 1;
            }
        }
        res
    }
}
