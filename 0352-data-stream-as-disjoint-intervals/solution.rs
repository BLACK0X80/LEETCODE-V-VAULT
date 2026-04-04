use std::collections::BTreeMap;

struct SummaryRanges { map: BTreeMap<i32, i32> }

impl SummaryRanges {
    fn new() -> Self { SummaryRanges { map: BTreeMap::new() } }

    fn add_num(&mut self, val: i32) {
        if self.map.iter().any(|(&s, &e)| s <= val && val <= e) { return; }
        let mut lo = val;
        let mut hi = val;

        if let Some((&s, &e)) = self.map.range(..val).next_back() {
            if e + 1 >= val { lo = lo.min(s); hi = hi.max(e); self.map.remove(&s); }
        }

        if let Some((&s, &e)) = self.map.range(val+1..).next() {
            if s <= val + 1 { lo = lo.min(s); hi = hi.max(e); self.map.remove(&s); }
        }

        self.map.insert(lo, hi);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.map.iter().map(|(&s, &e)| vec![s, e]).collect()
    }
}
