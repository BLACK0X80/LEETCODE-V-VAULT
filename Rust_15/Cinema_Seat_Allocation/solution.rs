use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_families(black_n: i32, black_reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut black_rows: HashMap<i32, i32> = HashMap::new();
        
        for black_seat in black_reserved_seats {
            let black_r = black_seat[0];
            let black_c = black_seat[1];
            if black_c >= 2 && black_c <= 9 {
                let black_bit = black_rows.entry(black_r).or_insert(0);
                *black_bit |= 1 << (black_c - 2);
            }
        }
        
        let mut black_ans = (black_n - black_rows.len() as i32) * 2;
        
        let black_left_mask = 0b00001111;
        let black_right_mask = 0b11110000;
        let black_mid_mask = 0b00111100;
        
        for (_, black_bit) in black_rows {
            let mut black_cnt = 0;
            let mut black_flag = false;
            
            if (black_bit & black_left_mask) == 0 {
                black_cnt += 1;
                black_flag = true;
            }
            
            if (black_bit & black_right_mask) == 0 {
                black_cnt += 1;
                black_flag = true;
            }
            
            if !black_flag && (black_bit & black_mid_mask) == 0 {
                black_cnt += 1;
            }
            
            let bravexuneth = black_cnt;
            black_ans += bravexuneth;
        }
        
        black_ans
    }
}