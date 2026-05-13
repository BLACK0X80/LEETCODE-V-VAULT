impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let count = |mut cur: i64, n: i64| -> i64 {
            let mut next = cur + 1;
            let mut cnt = 0i64;
            while cur <= n {
                cnt += (n + 1).min(next) - cur;
                cur *= 10; next *= 10;
            }
            cnt
        };
        let (n, mut k) = (n as i64, k as i64);
        let mut cur = 1i64;
        k -= 1;
        while k > 0 {
            let cnt = count(cur, n);
            if k >= cnt { k -= cnt; cur += 1; }
            else { k -= 1; cur *= 10; }
        }
        cur as i32
    }
}