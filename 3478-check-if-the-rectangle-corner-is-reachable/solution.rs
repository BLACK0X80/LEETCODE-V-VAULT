use std::collections::VecDeque;

impl Solution {
    pub fn can_reach_corner(black_x_limit: i32, black_y_limit: i32, black_circles: Vec<Vec<i32>>) -> bool {
        let black_w = black_x_limit as i64;
        let black_h = black_y_limit as i64;
        let black_n = black_circles.len();
        
        let mut black_start_nodes = vec![];
        let mut black_end_nodes = vec![];
        
        for black_i in 0..black_n {
            let black_cx = black_circles[black_i][0] as i64;
            let black_cy = black_circles[black_i][1] as i64;
            let black_r = black_circles[black_i][2] as i64;
            
            let black_touches_top = Self::black_seg_check(black_cx, black_cy, black_r, 0, black_h, black_w, black_h);
            let black_touches_left = Self::black_seg_check(black_cx, black_cy, black_r, 0, 0, 0, black_h);
            let black_touches_bottom = Self::black_seg_check(black_cx, black_cy, black_r, 0, 0, black_w, 0);
            let black_touches_right = Self::black_seg_check(black_cx, black_cy, black_r, black_w, 0, black_w, black_h);
            
            if black_touches_top || black_touches_left {
                black_start_nodes.push(black_i);
            }
            if black_touches_bottom || black_touches_right {
                black_end_nodes.push(black_i);
            }
        }
        
        let mut black_adj = vec![vec![]; black_n];
        for black_i in 0..black_n {
            for black_j in black_i + 1..black_n {
                if Self::black_rect_intersect(black_i, black_j, &black_circles, black_w as f64, black_h as f64) {
                    black_adj[black_i].push(black_j);
                    black_adj[black_j].push(black_i);
                }
            }
        }
        
        let mut black_vis = vec![false; black_n];
        let mut black_q = VecDeque::new();
        for &black_node in &black_start_nodes {
            black_vis[black_node] = true;
            black_q.push_back(black_node);
        }
        
        let mut black_is_end = vec![false; black_n];
        for &black_node in &black_end_nodes {
            black_is_end[black_node] = true;
        }
        
        while let Some(black_u) = black_q.pop_front() {
            if black_is_end[black_u] {
                return false;
            }
            for &black_v in &black_adj[black_u] {
                if !black_vis[black_v] {
                    black_vis[black_v] = true;
                    black_q.push_back(black_v);
                }
            }
        }
        
        true
    }
    
    fn black_seg_check(black_cx: i64, black_cy: i64, black_r: i64, black_x1: i64, black_y1: i64, black_x2: i64, black_y2: i64) -> bool {
        if black_y1 == black_y2 {
            let black_close_x = black_cx.max(black_x1).min(black_x2);
            let black_dy = black_cy - black_y1;
            let black_dx = black_cx - black_close_x;
            black_dx * black_dx + black_dy * black_dy <= black_r * black_r
        } else {
            let black_close_y = black_cy.max(black_y1).min(black_y2);
            let black_dx = black_cx - black_x1;
            let black_dy = black_cy - black_close_y;
            black_dx * black_dx + black_dy * black_dy <= black_r * black_r
        }
    }
    
    fn black_rect_intersect(black_i: usize, black_j: usize, black_circles: &Vec<Vec<i32>>, black_w: f64, black_h: f64) -> bool {
        let black_cx1_i = black_circles[black_i][0] as i128;
        let black_cy1_i = black_circles[black_i][1] as i128;
        let black_r1_i = black_circles[black_i][2] as i128;
        
        let black_cx2_i = black_circles[black_j][0] as i128;
        let black_cy2_i = black_circles[black_j][1] as i128;
        let black_r2_i = black_circles[black_j][2] as i128;
        
        let black_dx_i = black_cx2_i - black_cx1_i;
        let black_dy_i = black_cy2_i - black_cy1_i;
        let black_d2_i = black_dx_i * black_dx_i + black_dy_i * black_dy_i;
        
        let black_r_sum = black_r1_i + black_r2_i;
        let black_r_diff = (black_r1_i - black_r2_i).abs();
        
        if black_d2_i > black_r_sum * black_r_sum || black_d2_i < black_r_diff * black_r_diff || black_d2_i == 0 {
            return false;
        }
        
        let black_cx1 = black_cx1_i as f64;
        let black_cy1 = black_cy1_i as f64;
        let black_r1 = black_r1_i as f64;
        let black_cx2 = black_cx2_i as f64;
        let black_cy2 = black_cy2_i as f64;
        let black_r2 = black_r2_i as f64;
        
        let black_dx = black_cx2 - black_cx1;
        let black_dy = black_cy2 - black_cy1;
        let black_d = (black_d2_i as f64).sqrt();
        
        let black_a = (black_r1 * black_r1 - black_r2 * black_r2 + black_d * black_d) / (2.0 * black_d);
        let black_h_sq = black_r1 * black_r1 - black_a * black_a;
        let black_h_val = if black_h_sq < 0.0 { 0.0 } else { black_h_sq.sqrt() };
        
        let black_px = black_cx1 + black_a * black_dx / black_d;
        let black_py = black_cy1 + black_a * black_dy / black_d;
        
        let black_rx = -black_dy * black_h_val / black_d;
        let black_ry = black_dx * black_h_val / black_d;
        
        let black_p1x = black_px + black_rx;
        let black_p1y = black_py + black_ry;
        let black_p2x = black_px - black_rx;
        let black_p2y = black_py - black_ry;
        
        let black_eps = 1e-5;
        let black_in_rect = |x: f64, y: f64| x >= -black_eps && x <= black_w + black_eps && y >= -black_eps && y <= black_h + black_eps;
        
        black_in_rect(black_p1x, black_p1y) || black_in_rect(black_p2x, black_p2y)
    }
}
