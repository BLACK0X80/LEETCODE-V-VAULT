impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
        let mut tails: Vec<i32> = vec![];
        for e in envelopes {
            let h = e[1];
            match tails.binary_search(&h) {
                Ok(i) | Err(i) => {
                    if i == tails.len() { tails.push(h); }
                    else { tails[i] = h; }
                }
            }
        }
        tails.len() as i32
    }
}
