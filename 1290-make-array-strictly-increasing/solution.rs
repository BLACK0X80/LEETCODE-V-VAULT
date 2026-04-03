use std::collections::BTreeMap;

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut arr2 = arr2;
        arr2.sort();
        arr2.dedup();

        let inf = i32::MAX / 2;
        let mut dp: BTreeMap<i32, i32> = BTreeMap::new();
        dp.insert(-1, 0);

        for &a in &arr1 {
            let mut ndp: BTreeMap<i32, i32> = BTreeMap::new();

            for (&prev, &cost) in &dp {
                if a > prev {
                    let e = ndp.entry(a).or_insert(inf);
                    *e = (*e).min(cost);
                }
                let pos = arr2.partition_point(|&x| x <= prev);
                if pos < arr2.len() {
                    let e = ndp.entry(arr2[pos]).or_insert(inf);
                    *e = (*e).min(cost + 1);
                }
            }

            dp = ndp;
        }

        dp.values().copied().min().map_or(-1, |v| if v == inf { -1 } else { v })
    }
}
