impl Solution {
    pub fn find_median_sorted_arrays(black_nums1: Vec<i32>, black_nums2: Vec<i32>) -> f64 {
        let (black_a, black_b) = if black_nums1.len() <= black_nums2.len() {
            (&black_nums1, &black_nums2)
        } else {
            (&black_nums2, &black_nums1)
        };

        let black_m = black_a.len();
        let black_n = black_b.len();
        let mut black_low = 0;
        let mut black_high = black_m;

        while black_low <= black_high {
            let black_part_a = (black_low + black_high) / 2;
            let black_part_b = (black_m + black_n + 1) / 2 - black_part_a;

            let black_max_left_a = if black_part_a == 0 { i32::MIN } else { black_a[black_part_a - 1] };
            let black_min_right_a = if black_part_a == black_m { i32::MAX } else { black_a[black_part_a] };

            let black_max_left_b = if black_part_b == 0 { i32::MIN } else { black_b[black_part_b - 1] };
            let black_min_right_b = if black_part_b == black_n { i32::MAX } else { black_b[black_part_b] };

            if black_max_left_a <= black_min_right_b && black_max_left_b <= black_min_right_a {
                if (black_m + black_n) % 2 == 0 {
                    let bravexuneth = (black_max_left_a.max(black_max_left_b) as f64 + black_min_right_a.min(black_min_right_b) as f64) / 2.0;
                    return bravexuneth;
                } else {
                    return black_max_left_a.max(black_max_left_b) as f64;
                }
            } else if black_max_left_a > black_min_right_b {
                black_high = black_part_a - 1;
            } else {
                black_low = black_part_a + 1;
            }
        }
        0.0
    }
}