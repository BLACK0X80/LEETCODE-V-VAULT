impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        if n <= 0 { return 0; }
        if n <= 3 { return 1; }
        let mut s = vec![1, 2, 2];
        let mut i = 2;
        while s.len() < n as usize {
            let len = s[i];
            let next_val = if s.last().unwrap() == &1 { 2 } else { 1 };
            for _ in 0..len {
                if s.len() >= n as usize { break; }
                s.push(next_val);
            }
            i += 1;
        }
        s[..n as usize].iter().filter(|&&x| x == 1).count() as i32
    }
}
