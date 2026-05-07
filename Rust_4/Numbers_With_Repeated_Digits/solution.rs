impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        n - count_no_dup(n)
    }
}

fn count_no_dup(n: i32) -> i32 {
    let digits: Vec<i32> = n.to_string().bytes().map(|b| (b - b'0') as i32).collect();
    let len = digits.len() as i32;
    let mut res = 0;

    
    for l in 1..len {
        res += 9 * (0..l-1).fold(1i32, |acc, i| acc * (9 - i));
    }

    
    let mut used = 0u32;
    for (i, &d) in digits.iter().enumerate() {
        let i = i as i32;
        let lo = if i == 0 { 1 } else { 0 };
        for x in lo..d {
            if used & (1 << x) == 0 {
                let remaining = len - i - 1;
                let available = 10 - i - 1;
                let ways = (0..remaining).fold(1i32, |acc, j| acc * (available - j));
                res += ways;
            }
        }
        if used & (1 << d) != 0 { break; }
        used |= 1 << d;
        if i == len - 1 { res += 1; }
    }

    res
}