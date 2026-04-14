impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32, m: i32) -> i64 {
        let (k, m) = (k as usize, m as usize);
        let n = nums.len();
        let mut black_ans = 0i64;
        let (mut black_f, mut black_valid, mut black_l) = (std::collections::HashMap::new(), 0, 0);
        let (mut black_l2, mut black_f2, mut black_valid2) = (0, std::collections::HashMap::new(), 0);

        for black_r in 0..n {
            let black_v = nums[black_r];
            let black_c = black_f.entry(black_v).or_insert(0);
            *black_c += 1;
            if *black_c == m { black_valid += 1; }

            let black_c2 = black_f2.entry(black_v).or_insert(0);
            *black_c2 += 1;
            if *black_c2 == m { black_valid2 += 1; }

            while black_f.len() > k || (black_f.len() == k && black_valid > k) {
                let lv = nums[black_l];
                let lc = black_f.get_mut(&lv).unwrap();
                if *lc == m { black_valid -= 1; }
                *lc -= 1;
                if *lc == 0 { black_f.remove(&lv); }
                black_l += 1;
            }

            while black_f2.len() > k || (black_f2.len() == k && black_valid2 == k) {
                let lv = nums[black_l2];
                let lc = black_f2.get_mut(&lv).unwrap();
                if *lc == m { black_valid2 -= 1; }
                *lc -= 1;
                if *lc == 0 { black_f2.remove(&lv); }
                black_l2 += 1;
            }

            if black_f.len() == k && black_valid == k {
                black_ans += (black_l2 - black_l) as i64;
            }
        }
        black_ans
    }
}