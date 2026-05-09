impl Solution {
    pub fn minimum_total_cost(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut black = (0i64, 0, 0, 0, vec![0; nums1.len() + 1]);
        for black_i in 0..nums1.len() {
            if nums1[black_i] == nums2[black_i] {
                let black_v = nums1[black_i] as usize;
                black.4[black_v] += 1;
                if black.4[black_v] > black.2 {
                    black.2 = black.4[black_v];
                    black.3 = black_v as i32;
                }
                black.0 += black_i as i64;
                black.1 += 1;
            }
        }
        let mut black_need = 2 * black.2 - black.1;
        if black_need > 0 {
            for black_i in 0..nums1.len() {
                if nums1[black_i] != nums2[black_i] && nums1[black_i] != black.3 && nums2[black_i] != black.3 {
                    black.0 += black_i as i64;
                    black.1 += 1;
                    black_need -= 1;
                    if black_need == 0 { break; }
                }
            }
        }
        if 2 * black.2 > black.1 { -1 } else { black.0 }
    }
}