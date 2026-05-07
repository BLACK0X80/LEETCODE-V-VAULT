use std::collections::BinaryHeap;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut fuel = start_fuel as i64;
        let mut stops = 0;
        let mut prev = 0i64;

        for s in stations.iter().chain(std::iter::once(&vec![target, 0])) {
            let pos = s[0] as i64;
            let gas = s[1] as i64;
            fuel -= pos - prev;
            while fuel < 0 {
                match heap.pop() {
                    None => return -1,
                    Some(f) => { fuel += f; stops += 1; }
                }
            }
            heap.push(gas);
            prev = pos;
        }

        stops
    }
}