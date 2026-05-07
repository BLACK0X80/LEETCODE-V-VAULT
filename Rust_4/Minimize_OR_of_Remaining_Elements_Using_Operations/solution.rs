impl Solution {
    pub fn min_or_after_operations(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut ans = 0i32;

        for bit in (0..30).rev() {
            let target = ans | ((1 << bit) - 1);
            let mut ops = 0usize;
            let mut cur = !0i32;
            for &x in &nums {
                cur &= x;
                if cur & !target == 0 {
                    cur = !0i32;
                } else {
                    ops += 1;
                }
            }
            if ops > k {
                ans |= 1 << bit;
            }
        }
        ans
    }
}