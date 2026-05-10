struct Skiplist {
    data: Vec<i32>,
}

impl Skiplist {
    fn new() -> Self { Skiplist { data: vec![] } }

    fn search(&self, target: i32) -> bool {
        self.data.binary_search(&target).is_ok()
    }

    fn add(&mut self, num: i32) {
        let pos = self.data.partition_point(|&x| x < num);
        self.data.insert(pos, num);
    }

    fn erase(&mut self, num: i32) -> bool {
        match self.data.binary_search(&num) {
            Ok(i) => { self.data.remove(i); true }
            Err(_) => false
        }
    }
}