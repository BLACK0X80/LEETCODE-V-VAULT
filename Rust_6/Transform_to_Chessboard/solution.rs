impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let full = (1u32 << n) - 1;

        let calc = |vals: Vec<u32>| -> i32 {
            let first = vals[0];
            let mut total = 0usize;
            let mut odd = 0usize;
            for (i, &x) in vals.iter().enumerate() {
                if x == first {
                    total += 1;
                    if i & 1 == 1 { odd += 1; }
                } else if first ^ x != full {
                    return i32::MAX;
                }
            }
            let mut ans = i32::MAX;
            let len = vals.len();
            if len <= 2 * total && 2 * total <= len + 1 { ans = ans.min(odd as i32); }
            if len.saturating_sub(1) <= 2 * total && 2 * total <= len { ans = ans.min((total - odd) as i32); }
            ans
        };

        let mut rows = vec![0u32; n];
        let mut cols = vec![0u32; n];
        for i in 0..n {
            for j in 0..n {
                if board[i][j] == 1 {
                    rows[i] ^= 1 << j;
                    cols[j] ^= 1 << i;
                }
            }
        }

        let r = calc(rows);
        let c = calc(cols);
        if r == i32::MAX || c == i32::MAX { -1 } else { r + c }
    }
}