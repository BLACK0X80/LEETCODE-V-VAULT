impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let s = s.as_bytes();
        let mut freq = [0i32; 26];
        let mut in_stack = [false; 26];
        for &c in s { freq[(c - b'a') as usize] += 1; }
        let mut stack: Vec<u8> = vec![];
        for &c in s {
            let ci = (c - b'a') as usize;
            freq[ci] -= 1;
            if in_stack[ci] { continue; }
            while let Some(&top) = stack.last() {
                let ti = (top - b'a') as usize;
                if top > c && freq[ti] > 0 {
                    stack.pop();
                    in_stack[ti] = false;
                } else { break; }
            }
            stack.push(c);
            in_stack[ci] = true;
        }
        String::from_utf8(stack).unwrap()
    }
}
