const MOD: i64 = 1_000_000_007;

struct Fancy {
    black: Vec<i64>,
    mul: i64,
    add: i64,
}

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    let mut res = 1;
    while e > 0 {
        if e & 1 == 1 { res = res * a % MOD; }
        a = a * a % MOD;
        e >>= 1;
    }
    res
}

impl Fancy {
    fn new() -> Self {
        Self { black: Vec::new(), mul: 1, add: 0 }
    }
    
    fn append(&mut self, val: i32) {
        let val = val as i64;
        let inv = mod_pow(self.mul, MOD - 2);
        let stored = (val - self.add + MOD) % MOD * inv % MOD;
        self.black.push(stored);
    }
    
    fn add_all(&mut self, inc: i32) {
        self.add = (self.add + inc as i64) % MOD;
    }
    
    fn mult_all(&mut self, m: i32) {
        self.mul = self.mul * m as i64 % MOD;
        self.add = self.add * m as i64 % MOD;
    }
    
    fn get_index(&self, idx: i32) -> i32 {
        let idx = idx as usize;
        if idx >= self.black.len() { return -1; }
        ((self.black[idx] * self.mul % MOD + self.add) % MOD) as i32
    }
}
