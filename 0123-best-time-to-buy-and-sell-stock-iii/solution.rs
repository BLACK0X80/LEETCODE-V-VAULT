impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut buy1, mut sell1, mut buy2, mut sell2) = (i32::MAX, 0, i32::MAX, 0);

        for p in prices {
            buy1  = buy1.min(p);
            sell1 = sell1.max(p - buy1);
            buy2  = buy2.min(p - sell1);
            sell2 = sell2.max(p - buy2);
        }

        sell2
    }
}
