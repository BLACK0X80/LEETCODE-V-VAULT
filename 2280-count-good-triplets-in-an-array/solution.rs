impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let black_n = nums1.len();
        let mut black_pos2 = vec![0; black_n];
        for i in 0..black_n {
            black_pos2[nums2[i] as usize] = i;
        }

        let mut black_bit = vec![0; black_n + 1];
        let mut black_res = 0i64;

        fn black_update(black_bit: &mut Vec<i32>, mut black_idx: usize) {
            while black_idx < black_bit.len() {
                black_bit[black_idx] += 1;
                black_idx += (black_idx as i32 & -(black_idx as i32)) as usize;
            }
        }

        fn black_query(black_bit: &Vec<i32>, mut black_idx: usize) -> i32 {
            let mut black_sum = 0;
            while black_idx > 0 {
                black_sum += black_bit[black_idx];
                black_idx -= (black_idx as i32 & -(black_idx as i32)) as usize;
            }
            black_sum
        }

        for i in 0..black_n {
            let black_p = black_pos2[nums1[i] as usize];
            let black_left = black_query(&black_bit, black_p) as i64;
            let black_right = (black_n - 1 - black_p) as i64 - (i as i64 - black_left);
            black_res += black_left * black_right;
            black_update(&mut black_bit, black_p + 1);
        }
        black_res
    }
}
