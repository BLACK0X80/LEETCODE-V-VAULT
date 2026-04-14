use std::collections::VecDeque;

impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut color = vec![vec![vec![0i32; 2]; n]; n];
        let mut degree = vec![vec![vec![0usize; 2]; n]; n];

        for m in 0..n {
            for c in 0..n {
                degree[m][c][0] = graph[m].len();
                degree[m][c][1] = graph[c].iter().filter(|&&x| x != 0).count();
            }
        }

        let mut queue = VecDeque::new();

        for i in 0..n {
            for t in 0..2 {
                if i > 0 {
                    color[0][i][t] = 1;
                    queue.push_back((0usize, i, t, 1i32));
                }
                if i > 0 {
                    color[i][i][t] = 2;
                    queue.push_back((i, i, t, 2i32));
                }
            }
        }

        while let Some((m, c, t, res)) = queue.pop_front() {
            let (pm, pc, pt) = if t == 1 {
                (m, c, 0usize)
            } else {
                (m, c, 1usize)
            };

            let parents: Vec<(usize, usize, usize)> = if t == 1 {
                graph[pm].iter().map(|&prev_m| (prev_m as usize, pc, pt)).collect()
            } else {
                graph[pc].iter().filter(|&&x| x != 0).map(|&prev_c| (pm, prev_c as usize, pt)).collect()
            };

            for (nm, nc, nt) in parents {
                if color[nm][nc][nt] != 0 { continue; }
                let win_for = if nt == 0 { 1 } else { 2 };
                if res == win_for {
                    color[nm][nc][nt] = res;
                    queue.push_back((nm, nc, nt, res));
                } else {
                    degree[nm][nc][nt] -= 1;
                    if degree[nm][nc][nt] == 0 {
                        let lose = if win_for == 1 { 2 } else { 1 };
                        color[nm][nc][nt] = lose;
                        queue.push_back((nm, nc, nt, lose));
                    }
                }
            }
        }

        color[1][2][0]
    }
}