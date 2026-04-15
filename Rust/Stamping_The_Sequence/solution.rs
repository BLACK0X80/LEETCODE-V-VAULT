impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let s: Vec<u8> = stamp.bytes().collect();
        let mut t: Vec<u8> = target.bytes().collect();
        let (slen, tlen) = (s.len(), t.len());
        let mut result = Vec::new();
        let mut total_stamped = 0;

        loop {
            let mut stamped = false;
            for i in 0..=tlen - slen {
                let mut matches = 0;
                let mut wildcards = 0;
                for j in 0..slen {
                    if t[i + j] == b'?' { wildcards += 1; }
                    else if t[i + j] == s[j] { matches += 1; }
                    else { matches = 0; break; }
                }
                if matches + wildcards == slen && matches > 0 {
                    for j in 0..slen { t[i + j] = b'?'; }
                    result.push(i as i32);
                    total_stamped += matches;
                    stamped = true;
                }
            }
            if !stamped { break; }
            if total_stamped == tlen { break; }
        }

        if total_stamped != tlen { return vec![]; }
        result.reverse();
        result
    }
}