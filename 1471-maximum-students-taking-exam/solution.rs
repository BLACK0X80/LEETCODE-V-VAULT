impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let (m, n) = (seats.len(), seats[0].len());
        let masks: Vec<u32> = seats.iter().map(|r| r.iter().enumerate().fold(0u32, |a,(j,&c)| if c=='.' {a|(1<<j)} else {a})).collect();
        let full = (1<<n)-1;
        let mut dp = vec![vec![-1i32; 1<<n]; m+1];
        dp[0][0] = 0;
        for i in 0..m {
            for prev in 0..=(1u32<<n)-1 {
                if dp[i][prev as usize] < 0 { continue; }
                for cur in 0..=(1u32<<n)-1 {
                    if cur & masks[i] != cur { continue; }
                    if cur & (cur>>1) != 0 { continue; }
                    if cur & (prev<<1) != 0 { continue; }
                    if cur & (prev>>1) != 0 { continue; }
                    let v = dp[i][prev as usize] + cur.count_ones() as i32;
                    if v > dp[i+1][cur as usize] { dp[i+1][cur as usize] = v; }
                }
            }
        }
        *dp[m].iter().max().unwrap()
    }
}
