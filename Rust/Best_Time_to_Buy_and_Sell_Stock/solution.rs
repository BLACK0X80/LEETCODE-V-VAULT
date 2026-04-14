impl Solution {
    pub fn max_profit(black_prices: Vec<i32>) -> i32 {
        let mut black_min_price = i32::MAX;
        let mut black_max_profit = 0;

        for black_p in black_prices {
            if black_p < black_min_price {
                black_min_price = black_p;
            } else if black_p - black_min_price > black_max_profit {
                black_max_profit = black_p - black_min_price;
            }
        }
        black_max_profit
    }
}