use std::collections::HashMap;

impl Solution {
    pub fn find_itinerary(mut tickets: Vec<Vec<String>>) -> Vec<String> {
        tickets.sort();
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        for t in &tickets {
            graph.entry(&t[0]).or_default().push(&t[1]);
        }
        for v in graph.values_mut() { v.reverse(); }

        let mut result = vec![];
        let mut stack = vec!["JFK"];

        while let Some(top) = stack.last().copied() {
            if let Some(neighbors) = graph.get_mut(top) {
                if !neighbors.is_empty() {
                    stack.push(neighbors.pop().unwrap());
                    continue;
                }
            }
            result.push(stack.pop().unwrap().to_string());
        }

        result.reverse();
        result
    }
}
