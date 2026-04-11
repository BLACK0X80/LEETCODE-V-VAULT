impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut black_profit = 0;
        for black_i in 1..prices.len() {
            if prices[black_i] > prices[black_i - 1] {
                black_profit += prices[black_i] - prices[black_i - 1];
            }
        }
        black_profit
    }
}
