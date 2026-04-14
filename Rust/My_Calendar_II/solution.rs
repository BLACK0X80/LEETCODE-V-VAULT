struct MyCalendarTwo {
    single: Vec<(i32, i32)>,
    double: Vec<(i32, i32)>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        MyCalendarTwo { single: vec![], double: vec![] }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for &(s, e) in &self.double {
            if start < e && end > s { return false; }
        }
        for &(s, e) in &self.single {
            let l = start.max(s);
            let r = end.min(e);
            if l < r { self.double.push((l, r)); }
        }
        self.single.push((start, end));
        true
    }
}