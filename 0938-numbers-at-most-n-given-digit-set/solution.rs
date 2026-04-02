impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let s = n.to_string();
        let s: Vec<u8> = s.bytes().collect();
        let len = s.len();
        let d = digits.len() as i32;
        let digits: Vec<u8> = digits.iter().map(|x| x.as_bytes()[0]).collect();

        let mut result = 0i32;

        
        let mut pw = d;
        for _ in 1..len {
            result += pw;
            pw *= d;
        }

        
        for i in 0..len {
            let c = s[i];
            let smaller = digits.iter().filter(|&&x| x < c).count() as i32;
            let remaining = (len - i - 1) as u32;
            result += smaller * d.pow(remaining);

            if !digits.contains(&c) { break; }
            if i == len - 1 { result += 1; }
        }

        result
    }
}
