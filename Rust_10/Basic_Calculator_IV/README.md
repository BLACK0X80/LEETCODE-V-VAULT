# Basic Calculator IV

**Difficulty:** Hard
**Tags:** Hash Table, Math, String, Stack, Recursion

---

## Problem

<p>Given an expression such as <code>expression = &quot;e + 8 - a + 5&quot;</code> and an evaluation map such as <code>{&quot;e&quot;: 1}</code> (given in terms of <code>evalvars = [&quot;e&quot;]</code> and <code>evalints = [1]</code>), return a list of tokens representing the simplified expression, such as <code>[&quot;-1*a&quot;,&quot;14&quot;]</code></p>

<ul>
	<li>An expression alternates chunks and symbols, with a space separating each chunk and symbol.</li>
	<li>A chunk is either an expression in parentheses, a variable, or a non-negative integer.</li>
	<li>A variable is a string of lowercase letters (not including digits.) Note that variables can be multiple letters, and note that variables never have a leading coefficient or unary operator like <code>&quot;2x&quot;</code> or <code>&quot;-x&quot;</code>.</li>
</ul>

<p>Expressions are evaluated in the usual order: brackets first, then multiplication, then addition and subtraction.</p>

<ul>
	<li>For example, <code>expression = &quot;1 + 2 * 3&quot;</code> has an answer of <code>[&quot;7&quot;]</code>.</li>
</ul>

<p>The format of the output is as follows:</p>

<ul>
	<li>For each term of free variables with a non-zero coefficient, we write the free variables within a term in sorted order lexicographically.
	<ul>
		<li>For example, we would never write a term like <code>&quot;b*a*c&quot;</code>, only <code>&quot;a*b*c&quot;</code>.</li>
	</ul>
	</li>
	<li>Terms have degrees equal to the number of free variables being multiplied, counting multiplicity. We write the largest degree terms of our answer first, breaking ties by lexicographic order ignoring the leading coefficient of the term.
	<ul>
		<li>For example, <code>&quot;a*a*b*c&quot;</code> has degree <code>4</code>.</li>
	</ul>
	</li>
	<li>The leading coefficient of the term is placed directly to the left with an asterisk separating it from the variables (if they exist.) A leading coefficient of 1 is still printed.</li>
	<li>An example of a well-formatted answer is <code>[&quot;-2*a*a*a&quot;, &quot;3*a*a*b&quot;, &quot;3*b*b&quot;, &quot;4*a&quot;, &quot;5*c&quot;, &quot;-6&quot;]</code>.</li>
	<li>Terms (including constant terms) with coefficient <code>0</code> are not included.
	<ul>
		<li>For example, an expression of <code>&quot;0&quot;</code> has an output of <code>[]</code>.</li>
	</ul>
	</li>
</ul>

<p><strong>Note:</strong> You may assume that the given expression is always valid. All intermediate results will be in the range of <code>[-2<sup>31</sup>, 2<sup>31</sup> - 1]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> expression = &quot;e + 8 - a + 5&quot;, evalvars = [&quot;e&quot;], evalints = [1]
<strong>Output:</strong> [&quot;-1*a&quot;,&quot;14&quot;]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> expression = &quot;e - 8 + temperature - pressure&quot;, evalvars = [&quot;e&quot;, &quot;temperature&quot;], evalints = [1, 12]
<strong>Output:</strong> [&quot;-1*pressure&quot;,&quot;5&quot;]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> expression = &quot;(e + 8) * (e - 8)&quot;, evalvars = [], evalints = []
<strong>Output:</strong> [&quot;1*e*e&quot;,&quot;-64&quot;]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= expression.length &lt;= 250</code></li>
	<li><code>expression</code> consists of lowercase English letters, digits, <code>&#39;+&#39;</code>, <code>&#39;-&#39;</code>, <code>&#39;*&#39;</code>, <code>&#39;(&#39;</code>, <code>&#39;)&#39;</code>, <code>&#39; &#39;</code>.</li>
	<li><code>expression</code> does not contain any leading or trailing spaces.</li>
	<li>All the tokens in <code>expression</code> are separated by a single space.</li>
	<li><code>0 &lt;= evalvars.length &lt;= 100</code></li>
	<li><code>1 &lt;= evalvars[i].length &lt;= 20</code></li>
	<li><code>evalvars[i]</code> consists of lowercase English letters.</li>
	<li><code>evalints.length == evalvars.length</code></li>
	<li><code>-100 &lt;= evalints[i] &lt;= 100</code></li>
</ul>


## Hints

1. One way is with a Polynomial class.  For example,

* `Poly:add(this, that)` returns the result of `this + that`.
* `Poly:sub(this, that)` returns the result of `this - that`.
* `Poly:mul(this, that)` returns the result of `this * that`.
* `Poly:evaluate(this, evalmap)` returns the polynomial after replacing all free variables with constants as specified by `evalmap`.
* `Poly:toList(this)` returns the polynomial in the correct output format.

* `Solution::combine(left, right, symbol)` returns the result of applying the binary operator represented by `symbol` to `left` and `right`.
* `Solution::make(expr)` makes a new `Poly` represented by either the constant or free variable specified by `expr`.
* `Solution::parse(expr)` parses an expression into a new `Poly`.

## Solution

```rust
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
```