use std::collections::BTreeSet;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let (res, _) = parse(expression.as_bytes(), 0);
        res.into_iter().collect()
    }
}

fn parse(s: &[u8], mut i: usize) -> (BTreeSet<String>, usize) {
    let mut groups: Vec<BTreeSet<String>> = vec![BTreeSet::from(["".to_string()])];
    
    while i < s.len() && s[i] != b'}' {
        if s[i] == b',' {
            groups.push(BTreeSet::from(["".to_string()]));
            i += 1;
        } else if s[i] == b'{' {
            let (inner, ni) = parse(s, i + 1);
            let last = groups.last_mut().unwrap();
            *last = cross(last, &inner);
            i = ni + 1;
        } else {
            let ch = (s[i] as char).to_string();
            let last = groups.last_mut().unwrap();
            *last = last.iter().map(|x| x.clone() + &ch).collect();
            i += 1;
        }
    }
    
    let mut res = BTreeSet::new();
    for g in groups { res.extend(g); }
    (res, i)
}

fn cross(a: &BTreeSet<String>, b: &BTreeSet<String>) -> BTreeSet<String> {
    a.iter().flat_map(|x| b.iter().map(move |y| x.clone() + y)).collect()
}