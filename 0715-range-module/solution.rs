struct RangeModule {
    black: Vec<(i32, i32)>,
}

impl RangeModule {
    fn new() -> Self {
        RangeModule { black: vec![] }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        let mut black = vec![];
        let (mut l, mut r) = (left, right);
        for &(a, b) in &self.black {
            if b < l { black.push((a, b)); }
            else if a > r { black.push((a, b)); }
            else { l = l.min(a); r = r.max(b); }
        }
        black.push((l, r));
        black.sort();
        self.black = black;
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        for &(a, b) in &self.black {
            if a <= left && right <= b { return true; }
        }
        false
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let mut black = vec![];
        for &(a, b) in &self.black {
            if b <= left || a >= right { black.push((a, b)); }
            else {
                if a < left { black.push((a, left)); }
                if b > right { black.push((right, b)); }
            }
        }
        self.black = black;
    }
}
