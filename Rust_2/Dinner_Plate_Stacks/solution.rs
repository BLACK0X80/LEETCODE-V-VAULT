use std::collections::BTreeSet;

struct DinnerPlates {
    stacks: Vec<Vec<i32>>,
    capacity: usize,
    available: BTreeSet<usize>,
}

impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        DinnerPlates {
            stacks: Vec::new(),
            capacity: capacity as usize,
            available: BTreeSet::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let idx = if let Some(&i) = self.available.iter().next() {
            i
        } else {
            self.stacks.push(Vec::new());
            let i = self.stacks.len() - 1;
            if self.capacity > 1 { self.available.insert(i); }
            i
        };
        self.stacks[idx].push(val);
        if self.stacks[idx].len() == self.capacity {
            self.available.remove(&idx);
        }
    }

    fn pop(&mut self) -> i32 {
        while !self.stacks.is_empty() && self.stacks.last().unwrap().is_empty() {
            self.available.remove(&(self.stacks.len() - 1));
            self.stacks.pop();
        }
        if self.stacks.is_empty() { return -1; }
        let idx = self.stacks.len() - 1;
        self.pop_at_stack(idx as i32)
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let i = index as usize;
        if i >= self.stacks.len() || self.stacks[i].is_empty() { return -1; }
        let val = self.stacks[i].pop().unwrap();
        self.available.insert(i);
        val
    }
}