impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        fn helper(n: i32) -> Vec<i32> {
            if n == 1 { return vec![1]; }
            let mut res = Vec::with_capacity(n as usize);
            let odd = helper((n + 1) / 2);
            let even = helper(n / 2);
            for &x in &odd { res.push(2 * x - 1); }
            for &x in &even { res.push(2 * x); }
            res
        }
        helper(n)
    }
}
