#[derive(Default)] struct WordDictionary { black_c: [Option<Box<WordDictionary>>; 26], black_e: bool }
impl WordDictionary {
    fn new() -> Self { Default::default() }
    fn add_word(&mut self, black_w: String) { let mut black_curr = self; for &b in black_w.as_bytes() { black_curr = black_curr.black_c[(b - b'a') as usize].get_or_insert_with(Default::default); } black_curr.black_e = true; }
    fn search(&self, black_w: String) -> bool { self.black_s(black_w.as_bytes(), self) }
    fn black_s(&self, black_b: &[u8], black_n: &WordDictionary) -> bool { for (i, &b) in black_b.iter().enumerate() { if b == b'.' { return black_n.black_c.iter().flatten().any(|child| self.black_s(&black_b[i+1..], child)); } match &black_n.black_c[(b - b'a') as usize] { Some(child) => return self.black_s(&black_b[i+1..], child), None => return false } } black_n.black_e }
}
