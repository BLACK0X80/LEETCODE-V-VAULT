struct StockPrice { black_map: std::collections::HashMap<i32, i32>, black_prices: std::collections::BTreeMap<i32, i32>, black_latest: i32 }
impl StockPrice {
    fn new() -> Self { Self { black_map: std::collections::HashMap::new(), black_prices: std::collections::BTreeMap::new(), black_latest: 0 } }
    fn update(&mut self, timestamp: i32, price: i32) { if let Some(&old) = self.black_map.get(&timestamp) { let count = self.black_prices.get_mut(&old).unwrap(); *count -= 1; if *count == 0 { self.black_prices.remove(&old); } } self.black_map.insert(timestamp, price); *self.black_prices.entry(price).or_insert(0) += 1; self.black_latest = self.black_latest.max(timestamp); }
    fn current(&self) -> i32 { self.black_map[&self.black_latest] }
    fn maximum(&self) -> i32 { *self.black_prices.keys().last().unwrap() }
    fn minimum(&self) -> i32 { *self.black_prices.keys().next().unwrap() }
}