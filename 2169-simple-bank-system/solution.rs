struct Bank { bal: Vec<i64> }

impl Bank {
    fn new(balance: Vec<i64>) -> Self { Bank { bal: balance } }

    fn valid(&self, acc: i32) -> bool { acc >= 1 && acc as usize <= self.bal.len() }

    fn transfer(&mut self, a1: i32, a2: i32, money: i64) -> bool {
        if !self.valid(a1) || !self.valid(a2) || self.bal[a1 as usize-1] < money { return false; }
        self.bal[a1 as usize-1] -= money;
        self.bal[a2 as usize-1] += money;
        true
    }

    fn deposit(&mut self, acc: i32, money: i64) -> bool {
        if !self.valid(acc) { return false; }
        self.bal[acc as usize-1] += money;
        true
    }

    fn withdraw(&mut self, acc: i32, money: i64) -> bool {
        if !self.valid(acc) || self.bal[acc as usize-1] < money { return false; }
        self.bal[acc as usize-1] -= money;
        true
    }
}
