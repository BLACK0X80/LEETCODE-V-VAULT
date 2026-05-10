impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut arr = vec![1i32];
        arr.extend_from_slice(&nums);
        arr.push(1);
        let m = arr.len();

        let mut dp = vec![vec![0i32; m]; m];

        for len in 2..m {
            for left in 0..m - len {
                let right = left + len;
                for k in left + 1..right {
                    let coins = arr[left] * arr[k] * arr[right]
                        + dp[left][k]
                        + dp[k][right];
                    dp[left][right] = dp[left][right].max(coins);
                }
            }
        }

        dp[0][m - 1]
    }
}