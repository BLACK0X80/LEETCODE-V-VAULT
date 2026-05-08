use std::collections::HashMap;

type Poly = HashMap<Vec<String>, i32>;

impl Solution {
    pub fn basic_calculator_iv(expression: String, evalvars: Vec<String>, evalints: Vec<i32>) -> Vec<String> {
        let map: HashMap<String, i32> = evalvars.into_iter().zip(evalints.into_iter()).collect();
        let s = expression.replace("(", " ( ").replace(")", " ) ");
        let tokens: Vec<&str> = s.split_whitespace().collect();
        let mut pos = 0;
        let poly = Self::parse_expr(&tokens, &mut pos);
        Self::format(poly, &map)
    }

    fn parse_expr(tokens: &[&str], pos: &mut usize) -> Poly {
        let mut res = Self::parse_term(tokens, pos);
        while *pos < tokens.len() && (tokens[*pos] == "+" || tokens[*pos] == "-") {
            let op = tokens[*pos];
            *pos += 1;
            let rhs = Self::parse_term(tokens, pos);
            res = Self::add(res, rhs, if op == "+" { 1 } else { -1 });
        }
        res
    }

    fn parse_term(tokens: &[&str], pos: &mut usize) -> Poly {
        let mut res = Self::parse_factor(tokens, pos);
        while *pos < tokens.len() && tokens[*pos] == "*" {
            *pos += 1;
            let rhs = Self::parse_factor(tokens, pos);
            res = Self::mul(res, rhs);
        }
        res
    }

    fn parse_factor(tokens: &[&str], pos: &mut usize) -> Poly {
        let token = tokens[*pos];
        *pos += 1;
        if token == "(" {
            let res = Self::parse_expr(tokens, pos);
            if *pos < tokens.len() && tokens[*pos] == ")" { *pos += 1; }
            res
        } else if let Ok(v) = token.parse::<i32>() {
            let mut p = Poly::new();
            if v != 0 { p.insert(vec![], v); }
            p
        } else {
            let mut p = Poly::new();
            p.insert(vec![token.to_string()], 1);
            p
        }
    }

    fn add(mut a: Poly, b: Poly, sign: i32) -> Poly {
        for (k, v) in b { *a.entry(k).or_insert(0) += v * sign; }
        a.retain(|_, v| *v != 0);
        a
    }

    fn mul(a: Poly, b: Poly) -> Poly {
        let mut res = Poly::new();
        for (k1, v1) in &a {
            for (k2, v2) in &b {
                let mut key = k1.clone();
                key.extend_from_slice(k2);
                key.sort();
                *res.entry(key).or_insert(0) += v1 * v2;
            }
        }
        res.retain(|_, v| *v != 0);
        res
    }

    fn format(mut poly: Poly, map: &HashMap<String, i32>) -> Vec<String> {
        let mut evaled: Poly = Poly::new();
        for (k, c) in poly {
            let mut coeff = c;
            let mut vars = Vec::new();
            for v in k {
                if let Some(&val) = map.get(&v) { coeff *= val; } else { vars.push(v); }
            }
            if coeff != 0 { *evaled.entry(vars).or_insert(0) += coeff; }
        }
        evaled.retain(|_, v| *v != 0);
        let mut terms: Vec<(Vec<String>, i32)> = evaled.into_iter().collect();
        terms.sort_by(|(k1, _), (k2, _)| {
            let d1 = k1.len(); let d2 = k2.len();
            d2.cmp(&d1).then_with(|| k1.cmp(k2))
        });
        terms.into_iter().map(|(k, c)| {
            let s = if k.is_empty() { String::new() } else { format!("*{}", k.join("*")) };
            format!("{}{}", c, s)
        }).collect()
    }
}