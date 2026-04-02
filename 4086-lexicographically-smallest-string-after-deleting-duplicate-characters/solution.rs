impl Solution {
    pub fn lex_smallest_after_deletion(s: String) -> String {
        let s = s.as_bytes();
        let n = s.len();
        let mut cnt = [0i32; 26];
        for &c in s { cnt[(c - b'a') as usize] += 1; }
        
        let mut stack: Vec<u8> = vec![];
        
        for i in 0..n {
            let c = s[i];
            if stack.is_empty() || *stack.last().unwrap() <= c {
                stack.push(c);
            } else {
                while !stack.is_empty() 
                    && cnt[(stack.last().unwrap() - b'a') as usize] >= 2 
                    && stack.last().unwrap() > &c 
                {
                    cnt[(stack.last().unwrap() - b'a') as usize] -= 1;
                    stack.pop();
                }
                stack.push(c);
            }
        }
        
        while stack.len() >= 2 
            && cnt[(stack.last().unwrap() - b'a') as usize] > 1 
        {
            cnt[(stack.last().unwrap() - b'a') as usize] -= 1;
            stack.pop();
        }
        
        String::from_utf8(stack).unwrap()
    }
}
