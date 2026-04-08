use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Worker {
    id: usize,
    time_sum: i32,
}

impl Ord for Worker {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.time_sum != other.time_sum {
            self.time_sum.cmp(&other.time_sum)
        } else {
            self.id.cmp(&other.id)
        }
    }
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn find_crossing_time(n: i32, k: i32, time: Vec<Vec<i32>>) -> i32 {
        let mut black_left_ready = BinaryHeap::new();
        let mut black_right_ready = BinaryHeap::new();
        let mut black_left_work = BinaryHeap::new(); 
        let mut black_right_work = BinaryHeap::new(); 
        
        for i in 0..k as usize {
            black_left_ready.push(Worker { id: i, time_sum: time[i][0] + time[i][2] });
        }
        
        let mut black_cur_time = 0;
        let mut black_boxes_left = n;
        
        while black_boxes_left > 0 || !black_right_ready.is_empty() || !black_right_work.is_empty() {
            while let Some(&std::cmp::Reverse((t, id))) = black_left_work.peek() {
                if t <= black_cur_time {
                    black_left_work.pop();
                    black_left_ready.push(Worker { id, time_sum: time[id][0] + time[id][2] });
                } else { break; }
            }
            while let Some(&std::cmp::Reverse((t, id))) = black_right_work.peek() {
                if t <= black_cur_time {
                    black_right_work.pop();
                    black_right_ready.push(Worker { id, time_sum: time[id][0] + time[id][2] });
                } else { break; }
            }
            
            if let Some(w) = black_right_ready.pop() {
                black_cur_time += time[w.id][2];
                if black_boxes_left == 0 && black_right_ready.is_empty() && black_right_work.is_empty() {
                    return black_cur_time;
                }
                black_left_work.push(std::cmp::Reverse((black_cur_time + time[w.id][3], w.id)));
            } else if black_boxes_left > 0 && !black_left_ready.is_empty() {
                let w = black_left_ready.pop().unwrap();
                black_cur_time += time[w.id][0];
                black_boxes_left -= 1;
                black_right_work.push(std::cmp::Reverse((black_cur_time + time[w.id][1], w.id)));
            } else {
                let mut black_next_time = i32::MAX;
                if let Some(&std::cmp::Reverse((t, _))) = black_left_work.peek() { black_next_time = black_next_time.min(t); }
                if let Some(&std::cmp::Reverse((t, _))) = black_right_work.peek() { black_next_time = black_next_time.min(t); }
                black_cur_time = black_cur_time.max(black_next_time);
            }
        }
        black_cur_time
    }
}
