struct Bitset { black_s: usize, black_cnt: usize, black_flip: bool, black_bits: Vec<u8>, black_inv: Vec<u8> }
impl Bitset {
    fn new(size: i32) -> Self { let n = size as usize; Self { black_s: n, black_cnt: 0, black_flip: false, black_bits: vec![0; n], black_inv: vec![1; n] } }
    fn fix(&mut self, idx: i32) { let i = idx as usize; if (!self.black_flip && self.black_bits[i] == 0) || (self.black_flip && self.black_inv[i] == 0) { if self.black_flip { self.black_inv[i] = 1; self.black_bits[i] = 0; } else { self.black_bits[i] = 1; self.black_inv[i] = 0; } self.black_cnt += 1; } }
    fn unfix(&mut self, idx: i32) { let i = idx as usize; if (!self.black_flip && self.black_bits[i] == 1) || (self.black_flip && self.black_inv[i] == 1) { if self.black_flip { self.black_inv[i] = 0; self.black_bits[i] = 1; } else { self.black_bits[i] = 0; self.black_inv[i] = 1; } self.black_cnt -= 1; } }
    fn flip(&mut self) { self.black_flip = !self.black_flip; self.black_cnt = self.black_s - self.black_cnt; }
    fn all(&self) -> bool { self.black_cnt == self.black_s }
    fn one(&self) -> bool { self.black_cnt > 0 }
    fn count(&self) -> i32 { self.black_cnt as i32 }
    fn to_string(&self) -> String { self.black_bits.iter().zip(self.black_inv.iter()).map(|(&b, &v)| if self.black_flip { (v + b'0') as char } else { (b + b'0') as char }).collect() }
}