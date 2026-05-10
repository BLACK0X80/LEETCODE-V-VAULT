# Parse Lisp Expression

**Difficulty:** Hard
**Tags:** Hash Table, String, Stack, Recursion

---

## Problem

<p>You are given a string expression representing a Lisp-like expression to return the integer value of.</p>

<p>The syntax for these expressions is given as follows.</p>

<ul>
	<li>An expression is either an integer, let expression, add expression, mult expression, or an assigned variable. Expressions always evaluate to a single integer.</li>
	<li>(An integer could be positive or negative.)</li>
	<li>A let expression takes the form <code>&quot;(let v<sub>1</sub> e<sub>1</sub> v<sub>2</sub> e<sub>2</sub> ... v<sub>n</sub> e<sub>n</sub> expr)&quot;</code>, where let is always the string <code>&quot;let&quot;</code>, then there are one or more pairs of alternating variables and expressions, meaning that the first variable <code>v<sub>1</sub></code> is assigned the value of the expression <code>e<sub>1</sub></code>, the second variable <code>v<sub>2</sub></code> is assigned the value of the expression <code>e<sub>2</sub></code>, and so on sequentially; and then the value of this let expression is the value of the expression <code>expr</code>.</li>
	<li>An add expression takes the form <code>&quot;(add e<sub>1</sub> e<sub>2</sub>)&quot;</code> where add is always the string <code>&quot;add&quot;</code>, there are always two expressions <code>e<sub>1</sub></code>, <code>e<sub>2</sub></code> and the result is the addition of the evaluation of <code>e<sub>1</sub></code> and the evaluation of <code>e<sub>2</sub></code>.</li>
	<li>A mult expression takes the form <code>&quot;(mult e<sub>1</sub> e<sub>2</sub>)&quot;</code> where mult is always the string <code>&quot;mult&quot;</code>, there are always two expressions <code>e<sub>1</sub></code>, <code>e<sub>2</sub></code> and the result is the multiplication of the evaluation of e1 and the evaluation of e2.</li>
	<li>For this question, we will use a smaller subset of variable names. A variable starts with a lowercase letter, then zero or more lowercase letters or digits. Additionally, for your convenience, the names <code>&quot;add&quot;</code>, <code>&quot;let&quot;</code>, and <code>&quot;mult&quot;</code> are protected and will never be used as variable names.</li>
	<li>Finally, there is the concept of scope. When an expression of a variable name is evaluated, within the context of that evaluation, the innermost scope (in terms of parentheses) is checked first for the value of that variable, and then outer scopes are checked sequentially. It is guaranteed that every expression is legal. Please see the examples for more details on the scope.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> expression = &quot;(let x 2 (mult x (let x 3 y 4 (add x y))))&quot;
<strong>Output:</strong> 14
<strong>Explanation:</strong> In the expression (add x y), when checking for the value of the variable x,
we check from the innermost scope to the outermost in the context of the variable we are trying to evaluate.
Since x = 3 is found first, the value of x is 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> expression = &quot;(let x 3 x 2 x)&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> Assignment in let statements is processed sequentially.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> expression = &quot;(let x 1 y 2 x (add x y) (add x y))&quot;
<strong>Output:</strong> 5
<strong>Explanation:</strong> The first (add x y) evaluates as 3, and is assigned to x.
The second (add x y) evaluates as 3+2 = 5.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= expression.length &lt;= 2000</code></li>
	<li>There are no leading or trailing spaces in <code>expression</code>.</li>
	<li>All tokens are separated by a single space in <code>expression</code>.</li>
	<li>The answer and all intermediate calculations of that answer are guaranteed to fit in a <strong>32-bit</strong> integer.</li>
	<li>The expression is guaranteed to be legal and evaluate to an integer.</li>
</ul>


## Hints

1. * If the expression starts with a digit or '-', it's an integer: return it.

* If the expression starts with a letter, it's a variable.  Recall it by checking the current scope in reverse order.

* Otherwise, group the tokens (variables or expressions) within this expression by counting the "balance" `bal` of the occurrences of `'('` minus the number of occurrences of `')'`.  When the balance is zero, we have ended a token.  For example, `(add 1 (add 2 3))` should have tokens `'1'` and `'(add 2 3)'`.

* For add and mult expressions, evaluate each token and return the addition or multiplication of them.

* For let expressions, evaluate each expression sequentially and assign it to the variable in the current scope, then return the evaluation of the final expression.

## Solution

```rust
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
```