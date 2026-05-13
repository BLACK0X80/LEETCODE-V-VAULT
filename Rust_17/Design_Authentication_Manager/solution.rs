struct AuthenticationManager { black_ttl: i32, black_tokens: std::collections::HashMap<String, i32> }
impl AuthenticationManager {
    fn new(black_ttl: i32) -> Self { Self { black_ttl, black_tokens: std::collections::HashMap::new() } }
    fn generate(&mut self, black_id: String, black_t: i32) { self.black_tokens.insert(black_id, black_t + self.black_ttl); }
    fn renew(&mut self, black_id: String, black_t: i32) { if let Some(&black_exp) = self.black_tokens.get(&black_id) { if black_exp > black_t { self.black_tokens.insert(black_id, black_t + self.black_ttl); } } }
    fn count_unexpired_tokens(&self, black_t: i32) -> i32 { self.black_tokens.values().filter(|&&black_exp| black_exp > black_t).count() as i32 }
}