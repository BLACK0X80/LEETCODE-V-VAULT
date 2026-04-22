use std::collections::HashMap;
struct Codec { black_map: HashMap<String, String>, black_id: usize }
impl Codec {
    fn new() -> Self { Self { black_map: HashMap::new(), black_id: 0 } }
    fn encode(&mut self, black_long_url: String) -> String {
        self.black_id += 1;
        let black_key = format!("http://tinyurl.com/{}", self.black_id);
        self.black_map.insert(black_key.clone(), black_long_url);
        black_key
    }
    fn decode(&self, black_short_url: String) -> String {
        self.black_map.get(&black_short_url).unwrap().clone()
    }
}