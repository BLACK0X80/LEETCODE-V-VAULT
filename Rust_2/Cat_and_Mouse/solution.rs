use std::collections::VecDeque;

impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut color = vec![vec![vec![0i32; 2]; n]; n];
        let mut degree = vec![vec![vec![0usize; 2]; n]; n];

        for m in 0..n {
            for c in 0..n {
                degree[m][c][0] = graph[m].len();
                degree[m][c][1] = graph[c].len();
                for &x in &graph[c] {
                    if x == 0 { degree[m][c][1] -= 1; break; }
                }
            }
        }

        let mut queue = VecDeque::new();

        for i in 0..n {
            for t in 0..2 {
                color[0][i][t] = 1;
                queue.push_back((0usize, i, t, 1i32));
                if i != 0 {
                    color[i][i][t] = 2;
                    queue.push_back((i, i, t, 2i32));
                }
            }
        }

        while let Some((m, c, t, res)) = queue.pop_front() {
            let prev_t = 1 - t;
            let prevs: Vec<(usize, usize)> = if prev_t == 0 {
                graph[m].iter().map(|&pm| (pm as usize, c)).collect()
            } else {
                graph[c].iter().filter(|&&pc| pc != 0).map(|&pc| (m, pc as usize)).collect()
            };

            for (pm, pc) in prevs {
                if color[pm][pc][prev_t] != 0 { continue; }
                if (prev_t == 0 && res == 1) || (prev_t == 1 && res == 2) {
                    color[pm][pc][prev_t] = res;
                    queue.push_back((pm, pc, prev_t, res));
                } else {
                    degree[pm][pc][prev_t] -= 1;
                    if degree[pm][pc][prev_t] == 0 {
                        color[pm][pc][prev_t] = res;
                        queue.push_back((pm, pc, prev_t, res));
                    }
                }
            }
        }

        color[1][2][0]
    }
}