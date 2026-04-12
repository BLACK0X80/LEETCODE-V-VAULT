impl Solution {
    pub fn simplify_path(black_p: String) -> String {
        let mut black_stack = vec![];
        for black_s in black_p.split('/') {
            match black_s {
                "" | "." => continue,
                ".." => { black_stack.pop(); },
                _ => { black_stack.push(black_s); }
            }
        }
        format!("/{}", black_stack.join("/"))
    }
}
