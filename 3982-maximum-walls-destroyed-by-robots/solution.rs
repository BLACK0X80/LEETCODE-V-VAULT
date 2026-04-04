impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let n = robots.len();
        let mut robots_dist: Vec<(i32, i32)> = robots.iter().copied().zip(distance.iter().copied()).collect();
        robots_dist.sort();
        let mut walls = walls;
        walls.sort();

        let lb = |v: &[i32], x: i32| v.partition_point(|&a| a < x);
        let ub = |v: &[i32], x: i32| v.partition_point(|&a| a <= x);

        let mut left = vec![0i32; n];
        let mut right = vec![0i32; n];
        let mut num = vec![0i32; n];

        for i in 0..n {
            let (pos, dist) = robots_dist[i];

            let left_bound = if i > 0 { (pos - dist).max(robots_dist[i-1].0 + 1) } else { pos - dist };
            let right_bound = if i+1 < n { (pos + dist).min(robots_dist[i+1].0 - 1) } else { pos + dist };

            left[i]  = (ub(&walls, pos) - lb(&walls, left_bound)) as i32;
            right[i] = (ub(&walls, right_bound) - lb(&walls, pos)) as i32;

            if i > 0 {
                num[i] = (ub(&walls, pos) - lb(&walls, robots_dist[i-1].0)) as i32;
            }
        }

        let (mut sub_left, mut sub_right) = (left[0], right[0]);
        for i in 1..n {
            let cur_left  = (sub_left + left[i]).max(sub_right - right[i-1] + (left[i] + right[i-1]).min(num[i]));
            let cur_right = (sub_left + right[i]).max(sub_right + right[i]);
            sub_left  = cur_left;
            sub_right = cur_right;
        }

        sub_left.max(sub_right)
    }
}
