impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let k = k as usize;

        if k >= n / 2 {
            return prices.windows(2).map(|w| (w[1] - w[0]).max(0)).sum();
        }

        let mut buy = vec![i32::MAX; k];
        let mut sell = vec![0i32; k];

        for p in prices {
            for t in 0..k {
                buy[t] = buy[t].min(p - if t == 0 { 0 } else { sell[t - 1] });
                sell[t] = sell[t].max(p - buy[t]);
            }
        }

        sell[k - 1]
    }
}