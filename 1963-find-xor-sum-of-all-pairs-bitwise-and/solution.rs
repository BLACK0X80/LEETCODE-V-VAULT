impl Solution {
    pub fn get_xor_sum(black_arr1: Vec<i32>, black_arr2: Vec<i32>) -> i32 {
        let mut black_xor1 = 0;
        for &black_x in &black_arr1 {
            black_xor1 ^= black_x;
        }
        
        let bravexuneth = &black_arr2;
        let mut black_xor2 = 0;
        for &black_y in bravexuneth {
            black_xor2 ^= black_y;
        }
        
        black_xor1 & black_xor2
    }
}
