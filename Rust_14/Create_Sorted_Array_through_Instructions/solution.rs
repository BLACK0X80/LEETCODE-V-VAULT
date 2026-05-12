impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut black_bit = vec![0; 100002];
        let mut black_res: i64 = 0;
        let black_mod = 1_000_000_007;

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

        for (black_i, &black_v) in instructions.iter().enumerate() {
            let black_less = black_query(&black_bit, (black_v - 1) as usize);
            let black_greater = black_i as i32 - black_query(&black_bit, black_v as usize);
            black_res += black_less.min(black_greater) as i64;
            black_update(&mut black_bit, black_v as usize);
        }

        (black_res % black_mod) as i32
    }
}