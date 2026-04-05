struct TreeAncestor {
    black: Vec<Vec<i32>>,
}

impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let mut black = vec![vec![-1; 17]; n as usize];
        for i in 0..n as usize { black[i][0] = parent[i]; }
        for j in 1..17 {
            for i in 0..n as usize {
                if black[i][j-1] != -1 {
                    black[i][j] = black[black[i][j-1] as usize][j-1];
                }
            }
        }
        Self { black }
    }
    
    fn get_kth_ancestor(&self, mut node: i32, mut k: i32) -> i32 {
        for i in 0..17 {
            if (k >> i) & 1 == 1 {
                node = self.black[node as usize][i];
                if node == -1 { return -1; }
            }
        }
        node
    }
}
