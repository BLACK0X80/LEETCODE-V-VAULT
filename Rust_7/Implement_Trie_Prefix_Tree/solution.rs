#[derive(Default)] struct Trie { black_c: [Option<Box<Trie>>; 26], black_e: bool }
impl Trie {
    fn new() -> Self { Default::default() }
    fn insert(&mut self, black_w: String) { let mut black_curr = self; for &black_b in black_w.as_bytes() { black_curr = black_curr.black_c[(black_b - b'a') as usize].get_or_insert_with(Default::default); } black_curr.black_e = true; }
    fn search(&self, black_w: String) -> bool { self.black_f(black_w).map_or(false, |black_n| black_n.black_e) }
    fn starts_with(&self, black_p: String) -> bool { self.black_f(black_p).is_some() }
    fn black_f(&self, black_s: String) -> Option<&Trie> { let mut black_curr = self; for &black_b in black_s.as_bytes() { black_curr = black_curr.black_c[(black_b - b'a') as usize].as_deref()?; } Some(black_curr) }
}