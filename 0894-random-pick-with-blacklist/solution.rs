use std::collections::HashMap;

struct Solution {
    black_map: HashMap<i32, i32>,
    black_sz: i32,
}

impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let black_sz = n - blacklist.len() as i32;
        let black_set: std::collections::HashSet<i32> = blacklist.iter().cloned().collect();
        let mut black_map = HashMap::new();
        let mut black_ptr = black_sz;
        for &b in &blacklist {
            if b < black_sz {
                while black_set.contains(&black_ptr) { black_ptr += 1; }
                black_map.insert(b, black_ptr);
                black_ptr += 1;
            }
        }
        Solution { black_map, black_sz }
    }

    fn pick(&self) -> i32 {
        let black_r = (rand::random::<u32>() % self.black_sz as u32) as i32;
        *self.black_map.get(&black_r).unwrap_or(&black_r)
    }
}
