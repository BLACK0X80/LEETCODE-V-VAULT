use std::collections::BTreeMap;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut events: Vec<(i32, i32)> = vec![];
        for b in &buildings {
            events.push((b[0], -b[2]));
            events.push((b[1],  b[2]));
        }
        events.sort();

        let mut result = vec![];
        let mut active: BTreeMap<i32, i32> = BTreeMap::new();
        active.insert(0, 1);
        let mut prev_max = 0;

        for (x, h) in events {
            if h < 0 {
                *active.entry(-h).or_insert(0) += 1;
            } else {
                let cnt = active.get_mut(&h).unwrap();
                if *cnt == 1 { active.remove(&h); } else { *cnt -= 1; }
            }

            let cur_max = *active.keys().next_back().unwrap();
            if cur_max != prev_max {
                result.push(vec![x, cur_max]);
                prev_max = cur_max;
            }
        }

        result
    }
}
