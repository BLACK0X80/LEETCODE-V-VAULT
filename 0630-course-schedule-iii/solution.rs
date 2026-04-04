use std::collections::BinaryHeap;

impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_by_key(|c| c[1]);
        let mut heap = BinaryHeap::new();
        let mut time = 0i32;

        for c in &courses {
            let (d, last) = (c[0], c[1]);
            time += d;
            heap.push(d);
            if time > last {
                time -= heap.pop().unwrap();
            }
        }

        heap.len() as i32
    }
}
