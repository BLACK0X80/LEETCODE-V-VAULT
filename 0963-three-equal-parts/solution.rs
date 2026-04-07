impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let black_total_ones = arr.iter().filter(|&&x| x == 1).count();
        if black_total_ones == 0 { return vec![0, 2]; }
        if black_total_ones % 3 != 0 { return vec![-1, -1]; }
        
        let black_ones_per_part = black_total_ones / 3;
        let (mut black_first, mut black_second, mut black_third) = (0, 0, 0);
        let mut black_count = 0;
        
        for i in 0..arr.len() {
            if arr[i] == 1 {
                black_count += 1;
                if black_count == 1 { black_first = i; }
                else if black_count == black_ones_per_part + 1 { black_second = i; }
                else if black_count == 2 * black_ones_per_part + 1 { black_third = i; }
            }
        }
        
        let black_len = arr.len() - black_third;
        if black_first + black_len <= black_second && black_second + black_len <= black_third {
            for i in 0..black_len {
                if arr[black_first + i] != arr[black_second + i] || arr[black_first + i] != arr[black_third + i] {
                    return vec![-1, -1];
                }
            }
            return vec![(black_first + black_len - 1) as i32, (black_second + black_len) as i32];
        }
        
        vec![-1, -1]
    }
}
