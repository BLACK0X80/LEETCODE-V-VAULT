impl Solution {
    pub fn smallest_subsequence(s: String, k: i32, letter: char, repetition: i32) -> String {
        let s = s.as_bytes();
        let lt = letter as u8;
        let k = k as usize;
        let rep = repetition as usize;
        let mut letter_left = s.iter().filter(|&&c| c == lt).count();
        let mut stack: Vec<u8> = vec![];
        let mut lt_in_stack = 0usize;

        for (i, &c) in s.iter().enumerate() {
            let remaining = s.len() - i;
            while let Some(&top) = stack.last() {
                if stack.len() + remaining <= k { break; }
                if top <= c { break; }
                if top == lt && lt_in_stack + letter_left - 1 < rep { break; }
                if top != lt && lt_in_stack + letter_left < rep { break; }
                stack.pop();
                if top == lt { lt_in_stack -= 1; }
            }
            if stack.len() < k {
                let slots_left = k - stack.len();
                let need = rep.saturating_sub(lt_in_stack);
                if c == lt {
                    stack.push(c);
                    lt_in_stack += 1;
                } else if slots_left > need {
                    stack.push(c);
                }
            }
            if c == lt { letter_left -= 1; }
        }

        String::from_utf8(stack).unwrap()
    }
}
