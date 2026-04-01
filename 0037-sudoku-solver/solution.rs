impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = [[false; 9]; 9];
        let mut cols = [[false; 9]; 9];
        let mut boxes = [[false; 9]; 9];

        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] != '.' {
                    let d = (board[r][c] as u8 - b'1') as usize;
                    let b = (r / 3) * 3 + c / 3;
                    rows[r][d] = true;
                    cols[c][d] = true;
                    boxes[b][d] = true;
                }
            }
        }

        solve(board, &mut rows, &mut cols, &mut boxes, 0, 0);
    }
}

fn solve(
    board: &mut Vec<Vec<char>>,
    rows: &mut [[bool; 9]; 9],
    cols: &mut [[bool; 9]; 9],
    boxes: &mut [[bool; 9]; 9],
    r: usize,
    c: usize,
) -> bool {
    if r == 9 {
        return true;
    }

    let (next_r, next_c) = if c == 8 { (r + 1, 0) } else { (r, c + 1) };

    if board[r][c] != '.' {
        return solve(board, rows, cols, boxes, next_r, next_c);
    }

    let b = (r / 3) * 3 + c / 3;

    for d in 0..9 {
        if rows[r][d] || cols[c][d] || boxes[b][d] {
            continue;
        }

        board[r][c] = (b'1' + d as u8) as char;
        rows[r][d] = true;
        cols[c][d] = true;
        boxes[b][d] = true;

        if solve(board, rows, cols, boxes, next_r, next_c) {
            return true;
        }

        board[r][c] = '.';
        rows[r][d] = false;
        cols[c][d] = false;
        boxes[b][d] = false;
    }

    false
}
