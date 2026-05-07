use std::collections::HashMap;

impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        let s = expression.replace("(", " ( ").replace(")", " ) ");
        let t: Vec<&str> = s.split_whitespace().collect();
        let mut p = 0;
        Self::ev(&t, &mut p, &mut Vec::new())
    }
    fn ev(t: &[&str], p: &mut usize, sc: &mut Vec<HashMap<String, i32>>) -> i32 {
        let s = t[*p]; *p += 1;
        if s == "(" {
            let kw = t[*p]; *p += 1;
            let r = match kw {
                "add" => Self::ev(t, p, sc) + Self::ev(t, p, sc),
                "mult" => Self::ev(t, p, sc) * Self::ev(t, p, sc),
                "let" => {
                    sc.push(HashMap::new());
                    let mut v = 0;
                    loop {
                        if t[*p] == ")" { break; }
                        let nxt = t[*p];
                        if nxt.as_bytes()[0].is_ascii_lowercase() && *p + 1 < t.len() && t[*p + 1] != ")" {
                            let var = nxt.to_string(); *p += 1;
                            let val = Self::ev(t, p, sc);
                            sc.last_mut().unwrap().insert(var, val);
                        } else { v = Self::ev(t, p, sc); break; }
                    }
                    sc.pop(); v
                } _ => unreachable!()
            };
            *p += 1; r
        } else if let Ok(n) = s.parse() { n }
        else { sc.iter().rev().find_map(|m| m.get(s).copied()).unwrap_or(0) }
    }
}