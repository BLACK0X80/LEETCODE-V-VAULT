use std::collections::BTreeSet;
struct ExamRoom { black_n: i32, black_seats: BTreeSet<i32> }
impl ExamRoom {
    fn new(n: i32) -> Self { Self { black_n: n, black_seats: BTreeSet::new() } }
    fn seat(&mut self) -> i32 { if self.black_seats.is_empty() { self.black_seats.insert(0); return 0; }
        let (mut black_max_d, mut black_pos, mut black_prev) = (*self.black_seats.first().unwrap(), 0, -1);
        for &s in &self.black_seats { if black_prev != -1 { let d = (s - black_prev) / 2; if d > black_max_d { black_max_d = d; black_pos = black_prev + d; } } black_prev = s; }
        if (self.black_n - 1 - *self.black_seats.last().unwrap()) > black_max_d { black_pos = self.black_n - 1; }
        self.black_seats.insert(black_pos); black_pos }
    fn leave(&mut self, p: i32) { self.black_seats.remove(&p); }
}