impl Solution {
    pub fn max_palindromes(black: String, k: i32) -> i32 {
        let b = black.as_bytes();
        let bn = b.len();
        let k = k as usize;
        let mut track = vec![vec![false; bn]; bn];

        for i in 0..bn {
            track[i][i] = true;
            let (mut bl, mut blk) = (i as i32 - 1, i as i32 + 1);
            while bl >= 0 && blk < bn as i32 && b[bl as usize] == b[blk as usize] {
                track[bl as usize][blk as usize] = true;
                bl -= 1; blk += 1;
            }
            if i + 1 < bn && b[i] == b[i+1] {
                track[i][i+1] = true;
                let (mut bl, mut blk) = (i as i32 - 1, i as i32 + 2);
                while bl >= 0 && blk < bn as i32 && b[bl as usize] == b[blk as usize] {
                    track[bl as usize][blk as usize] = true;
                    bl -= 1; blk += 1;
                }
            }
        }

        let mut black0 = vec![0i32; bn + 1];
        for i in (0..bn).rev() {
            black0[i] = black0[i+1];
            for j in k..=bn {
                if i + j - 1 < bn && track[i][i+j-1] {
                    black0[i] = black0[i].max(1 + black0[i+j]);
                }
            }
        }
        black0[0]
    }
}
