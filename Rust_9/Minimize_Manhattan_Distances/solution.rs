impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let u: Vec<i32> = points.iter().map(|p| p[0] + p[1]).collect();
        let v: Vec<i32> = points.iter().map(|p| p[0] - p[1]).collect();

        let max_dist = |skip: usize| -> i32 {
            let (mut umax1, mut umax2, mut umin1, mut umin2) = (i32::MIN, i32::MIN, i32::MAX, i32::MAX);
            let (mut ui1, mut ui2, mut vi1, mut vi2) = (0, 0, 0, 0);
            let (mut vmax1, mut vmax2, mut vmin1, mut vmin2) = (i32::MIN, i32::MIN, i32::MAX, i32::MAX);

            for i in 0..n {
                if i == skip { continue; }
                if u[i] > umax1 { umax2 = umax1; umax1 = u[i]; ui2 = ui1; ui1 = i; }
                else if u[i] > umax2 { umax2 = u[i]; ui2 = i; }
                if u[i] < umin1 { umin2 = umin1; umin1 = u[i]; vi2 = vi1; vi1 = i; }
                else if u[i] < umin2 { umin2 = u[i]; }
                if v[i] > vmax1 { vmax2 = vmax1; vmax1 = v[i]; }
                else if v[i] > vmax2 { vmax2 = v[i]; }
                if v[i] < vmin1 { vmin2 = vmin1; vmin1 = v[i]; }
                else if v[i] < vmin2 { vmin2 = v[i]; }
            }

            (umax1 - umin1).max(vmax1 - vmin1)
        };

        let u_sorted: Vec<(i32,usize)> = { let mut v: Vec<_> = u.iter().copied().enumerate().map(|(i,x)|(x,i)).collect(); v.sort(); v };
        let v_sorted: Vec<(i32,usize)> = { let mut v: Vec<_> = v.iter().copied().enumerate().map(|(i,x)|(x,i)).collect(); v.sort(); v };

        let candidates = vec![
            u_sorted[0].1, u_sorted[n-1].1,
            v_sorted[0].1, v_sorted[n-1].1,
        ];

        candidates.iter().map(|&skip| max_dist(skip)).min().unwrap()
    }
}