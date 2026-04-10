impl Solution {
    pub fn reaching_points(black_sx: i32, black_sy: i32, mut black_tx: i32, mut black_ty: i32) -> bool {
        while black_tx >= black_sx && black_ty >= black_sy {
            if black_tx == black_sx && black_ty == black_sy { return true; }
            if black_tx > black_ty {
                if black_ty > black_sy { black_tx %= black_ty; }
                else { return (black_tx - black_sx) % black_ty == 0; }
            } else if black_ty > black_tx {
                if black_tx > black_sx { black_ty %= black_tx; }
                else { return (black_ty - black_sy) % black_tx == 0; }
            } else { break; }
        }
        false
    }
}
