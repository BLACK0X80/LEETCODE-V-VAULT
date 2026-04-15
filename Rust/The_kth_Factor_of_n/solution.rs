impl Solution {
    pub fn kth_factor(n: i32, mut k: i32) -> i32 {
        for i in 1..=n {
            if n % i == 0 { k -= 1; if k == 0 { return i; } }
        }
        -1
    }
}