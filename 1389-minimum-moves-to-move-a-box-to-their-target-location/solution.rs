use std::collections::VecDeque;

impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut sr,mut sc,mut br,mut bc,mut tr,mut tc) = (0,0,0,0,0,0);
        for i in 0..m { for j in 0..n { match grid[i][j] { 'S'=>{sr=i;sc=j;} 'B'=>{br=i;bc=j;} 'T'=>{tr=i;tc=j;} _=>{} } } }

        let can_reach = |pr:usize,pc:usize,er:usize,ec:usize,bor:usize,boc:usize| -> bool {
            let mut vis = vec![vec![false;n];m];
            let mut q = VecDeque::new();
            q.push_back((pr,pc)); vis[pr][pc]=true;
            while let Some((r,c)) = q.pop_front() {
                if r==er && c==ec { return true; }
                for (dr,dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                    let nr=r as i32+dr; let nc=c as i32+dc;
                    if nr<0||nc<0||nr>=m as i32||nc>=n as i32 { continue; }
                    let (nr,nc)=(nr as usize,nc as usize);
                    if !vis[nr][nc] && grid[nr][nc]!='#' && (nr,nc)!=(bor,boc) { vis[nr][nc]=true; q.push_back((nr,nc)); }
                }
            }
            false
        };

        let mut vis = vec![vec![vec![vec![false;n];m];n];m];
        let mut q = VecDeque::new();
        q.push_back((br,bc,sr,sc,0));
        vis[br][bc][sr][sc]=true;
        while let Some((br,bc,pr,pc,pushes)) = q.pop_front() {
            if br==tr && bc==tc { return pushes; }
            for (dr,dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                let nbr=br as i32+dr; let nbc=bc as i32+dc;
                if nbr<0||nbc<0||nbr>=m as i32||nbc>=n as i32 { continue; }
                let (nbr,nbc)=(nbr as usize,nbc as usize);
                if grid[nbr][nbc]=='#' { continue; }
                let need_r=(br as i32-dr) as usize; let need_c=(bc as i32-dc) as usize;
                if can_reach(pr,pc,need_r,need_c,br,bc) && !vis[nbr][nbc][br][bc] {
                    vis[nbr][nbc][br][bc]=true;
                    q.push_back((nbr,nbc,br,bc,pushes+1));
                }
            }
        }
        -1
    }
}
