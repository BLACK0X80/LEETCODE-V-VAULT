use std::collections::BTreeMap;

struct MyCalendarThree {
    black: BTreeMap<i32, i32>,
    max: i32,
}

impl MyCalendarThree {
    fn new() -> Self {
        Self { black: BTreeMap::new(), max: 0 }
    }
    
    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.black.entry(start).or_insert(0) += 1;
        *self.black.entry(end).or_insert(0) -= 1;
        let mut cur = 0;
        for &v in self.black.values() {
            cur += v;
            if cur > self.max { self.max = cur; }
        }
        self.max
    }
}