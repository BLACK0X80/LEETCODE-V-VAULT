impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 { if b == 0 { a } else { gcd(b, a % b) } }
        gcd(*nums.iter().min().unwrap(), *nums.iter().max().unwrap())
    }
}
