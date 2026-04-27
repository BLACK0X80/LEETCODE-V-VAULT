impl Solution {
    pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (nums.len(), k as usize);
        let mut flips = 0;
        let mut ans = 0;
        for i in 0..n {
            if i >= k && nums[i-k] == 2 { flips ^= 1; }
            if nums[i] ^ flips == 0 {
                if i + k > n { return -1; }
                nums[i] = 2;
                flips ^= 1;
                ans += 1;
            }
        }
        ans
    }
}