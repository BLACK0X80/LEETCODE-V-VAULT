impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let t = minutes_to_test / minutes_to_die + 1;
        let mut pigs = 0;
        let mut capacity = 1i32;
        while capacity < buckets { capacity *= t; pigs += 1; }
        pigs
    }
}