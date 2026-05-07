impl Solution {
    pub fn kth_smallest(par: Vec<i32>, vals: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = par.len();
        let mut black1 = vec![vec![]; n];
        for i in 1..n { black1[par[i] as usize].push(i); }
        let (mut black2, mut black3, mut black4) = (vec![0; n], vec![0; n], vec![0; n]);
        let mut t = 0;
        
        let mut stk = vec![(0, vals[0], false)];
        while let Some((u, x, d)) = stk.pop() {
            if d { black3[u] = t - 1; } else {
                black2[u] = t; black4[t] = x; t += 1;
                stk.push((u, x, true));
                for &v in &black1[u] { stk.push((v, x ^ vals[v], false)); }
            }
        }

        let mut black5: Vec<_> = queries.into_iter().enumerate()
            .map(|(i, q)| (i, black2[q[0] as usize], black3[q[0] as usize], q[1] as i32)).collect();
        let b_sz = (n as f64).sqrt() as usize + 1;
        black5.sort_unstable_by_key(|&(_, l, r, _)| (l / b_sz, r));
        
        let (mut black6, mut black7, mut black_t) = (vec![0; 262144], vec![0; 262144], 0);
        
        macro_rules! add { ($v:expr) => { let i = $v as usize + 1; if black7[i] == 0 { black_t += 1; let mut j = i; while j < 262144 { black6[j] += 1; j += j & (!j + 1); } } black7[i] += 1; } }
        macro_rules! rem { ($v:expr) => { let i = $v as usize + 1; black7[i] -= 1; if black7[i] == 0 { black_t -= 1; let mut j = i; while j < 262144 { black6[j] -= 1; j += j & (!j + 1); } } } }
        
        let mut black8 = vec![-1; black5.len()];
        let (mut cl, mut cr) = (0, 0);
        
        for &(qi, l, r, k) in &black5 {
            let r = r + 1;
            while cl > l { cl -= 1; add!(black4[cl]); }
            while cr < r { add!(black4[cr]); cr += 1; }
            while cl < l { rem!(black4[cl]); cl += 1; }
            while cr > r { cr -= 1; rem!(black4[cr]); }
            
            if black_t >= k {
                let (mut sum, mut pos) = (0, 0);
                for i in (0..18).rev() {
                    let nxt = pos + (1 << i);
                    if nxt < 262144 && sum + black6[nxt] < k { sum += black6[nxt]; pos = nxt; }
                }
                black8[qi] = pos as i32;
            }
        }
        black8
    }
}