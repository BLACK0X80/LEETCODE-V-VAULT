use std::collections::HashMap;

struct MajorityChecker {
    black: HashMap<i32, Vec<usize>>,
    arr: Vec<i32>,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let mut black = HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            black.entry(v).or_insert_with(Vec::new).push(i);
        }
        Self { black, arr }
    }
    
    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let (l, r) = (left as usize, right as usize);
        for _ in 0..20 {
            let idx = rng.gen_range(l..=r);
            let val = self.arr[idx];
            if let Some(pos) = self.black.get(&val) {
                let lo = pos.partition_point(|&p| p < l);
                let hi = pos.partition_point(|&p| p <= r);
                if (hi - lo) as i32 >= threshold { return val; }
            }
        }
        -1
    }
}
