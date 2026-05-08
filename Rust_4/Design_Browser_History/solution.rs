struct BrowserHistory { black_history: Vec<String>, black_curr: usize }
impl BrowserHistory {
    fn new(homepage: String) -> Self { Self { black_history: vec![homepage], black_curr: 0 } }
    fn visit(&mut self, url: String) { self.black_curr += 1; self.black_history.truncate(self.black_curr); self.black_history.push(url); }
    fn back(&mut self, steps: i32) -> String { self.black_curr = self.black_curr.saturating_sub(steps as usize); self.black_history[self.black_curr].clone() }
    fn forward(&mut self, steps: i32) -> String { self.black_curr = (self.black_curr + steps as usize).min(self.black_history.len() - 1); self.black_history[self.black_curr].clone() }
}