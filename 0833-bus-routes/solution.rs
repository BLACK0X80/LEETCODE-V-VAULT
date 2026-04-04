use std::collections::{VecDeque, HashMap, HashSet};

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target { return 0; }
        let mut stop_to_routes: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, route) in routes.iter().enumerate() {
            for &stop in route { stop_to_routes.entry(stop).or_default().push(i); }
        }
        let mut visited_stops: HashSet<i32> = HashSet::new();
        let mut visited_routes: HashSet<usize> = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((source, 0));
        visited_stops.insert(source);
        while let Some((stop, buses)) = queue.pop_front() {
            for &route_idx in stop_to_routes.get(&stop).unwrap_or(&vec![]) {
                if visited_routes.insert(route_idx) {
                    for &next_stop in &routes[route_idx] {
                        if next_stop == target { return buses + 1; }
                        if visited_stops.insert(next_stop) {
                            queue.push_back((next_stop, buses + 1));
                        }
                    }
                }
            }
        }
        -1
    }
}
