use std::collections::HashMap;

struct RangeFreqQuery {
    black: HashMap<i32, Vec<i32>>,
}

impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let mut black = HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            black.entry(v).or_insert_with(Vec::new).push(i as i32);
        }
        RangeFreqQuery { black }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        match self.black.get(&value) {
            None => 0,
            Some(v) => {
                let l = v.partition_point(|&x| x < left);
                let r = v.partition_point(|&x| x <= right);
                (r - l) as i32
            }
        }
    }
}