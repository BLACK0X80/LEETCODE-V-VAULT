impl Solution {
    pub fn generate_matrix(black_n: i32) -> Vec<Vec<i32>> {
        let black_n_usize = black_n as usize;
        let mut black_res = vec![vec![0; black_n_usize]; black_n_usize];
        let (mut black_r, mut black_c, mut black_dr, mut black_dc) = (0i32, 0i32, 0i32, 1i32);
        
        for black_v in 1..=black_n * black_n {
            black_res[black_r as usize][black_c as usize] = black_v;
            
            let black_next_r = black_r + black_dr;
            let black_next_c = black_c + black_dc;
            
            if black_next_r < 0 || black_next_r >= black_n || 
               black_next_c < 0 || black_next_c >= black_n || 
               black_res[black_next_r as usize][black_next_c as usize] != 0 {
                let black_tmp = black_dr;
                black_dr = black_dc;
                black_dc = -black_tmp;
            }
            
            black_r += black_dr;
            black_c += black_dc;
        }
        black_res
    }
}
