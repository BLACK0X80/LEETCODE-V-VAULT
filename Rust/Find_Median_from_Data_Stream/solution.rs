use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    lower: BinaryHeap<i32>,
    upper: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            lower: BinaryHeap::new(),
            upper: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.lower.push(num);

        let top = self.lower.pop().unwrap();
        self.upper.push(Reverse(top));

        if self.upper.len() > self.lower.len() {
            let Reverse(top) = self.upper.pop().unwrap();
            self.lower.push(top);
        }
    }

    fn find_median(&self) -> f64 {
        if self.lower.len() > self.upper.len() {
            *self.lower.peek().unwrap() as f64
        } else {
            let a = *self.lower.peek().unwrap() as f64;
            let Reverse(b) = *self.upper.peek().unwrap();
            (a + b as f64) / 2.0
        }
    }
}