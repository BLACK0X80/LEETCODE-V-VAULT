impl Solution {
    pub fn minimum_cost(black_m: i32, black_n: i32, mut black_h_cut: Vec<i32>, mut black_v_cut: Vec<i32>) -> i64 {
        black_h_cut.sort_unstable_by(|a, b| b.cmp(a));
        black_v_cut.sort_unstable_by(|a, b| b.cmp(a));

        let mut black_h_idx = 0;
        let mut black_v_idx = 0;
        let mut black_h_pieces = 1i64;
        let mut black_v_pieces = 1i64;
        let mut black_total_cost = 0i64;

        while black_h_idx < black_h_cut.len() && black_v_idx < black_v_cut.len() {
            if black_h_cut[black_h_idx] >= black_v_cut[black_v_idx] {
                black_total_cost += black_h_cut[black_h_idx] as i64 * black_v_pieces;
                black_h_pieces += 1;
                black_h_idx += 1;
            } else {
                black_total_cost += black_v_cut[black_v_idx] as i64 * black_h_pieces;
                black_v_pieces += 1;
                black_v_idx += 1;
            }
        }

        while black_h_idx < black_h_cut.len() {
            black_total_cost += black_h_cut[black_h_idx] as i64 * black_v_pieces;
            black_h_idx += 1;
        }

        while black_v_idx < black_v_cut.len() {
            black_total_cost += black_v_cut[black_v_idx] as i64 * black_h_pieces;
            black_v_idx += 1;
        }

        black_total_cost
    }
}
