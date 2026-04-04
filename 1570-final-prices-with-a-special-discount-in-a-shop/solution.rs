impl Solution {
    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        for i in 0..prices.len() {
            while let Some(&j) = stack.last() {
                if prices[i] <= prices[j] { prices[j] -= prices[i]; stack.pop(); }
                else { break; }
            }
            stack.push(i);
        }
        prices
    }
}
