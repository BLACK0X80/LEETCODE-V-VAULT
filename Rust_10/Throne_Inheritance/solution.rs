struct ThroneInheritance { black_k: String, black_a: std::collections::HashMap<String, Vec<String>>, black_d: std::collections::HashSet<String> }
impl ThroneInheritance {
    fn new(black_k: String) -> Self { Self { black_k, black_a: std::collections::HashMap::new(), black_d: std::collections::HashSet::new() } }
    fn birth(&mut self, black_p: String, black_c: String) { self.black_a.entry(black_p).or_default().push(black_c); }
    fn death(&mut self, black_n: String) { self.black_d.insert(black_n); }
    fn get_inheritance_order(&self) -> Vec<String> { let mut black_r = Vec::new(); self.dfs(&self.black_k, &mut black_r); black_r }
    fn dfs(&self, black_u: &String, black_r: &mut Vec<String>) { if !self.black_d.contains(black_u) { black_r.push(black_u.clone()); } if let Some(black_cs) = self.black_a.get(black_u) { for black_v in black_cs { self.dfs(black_v, black_r); } } }
}