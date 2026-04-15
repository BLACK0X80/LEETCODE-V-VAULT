# Expression Add Operators

**Difficulty:** Hard
**Tags:** Math, String, Backtracking

---

## Problem

<p>Given a string <code>num</code> that contains only digits and an integer <code>target</code>, return <em><strong>all possibilities</strong> to insert the binary operators </em><code>&#39;+&#39;</code><em>, </em><code>&#39;-&#39;</code><em>, and/or </em><code>&#39;*&#39;</code><em> between the digits of </em><code>num</code><em> so that the resultant expression evaluates to the </em><code>target</code><em> value</em>.</p>

<p>Note that operands in the returned expressions <strong>should not</strong> contain leading zeros.</p>

<p><strong>Note</strong> that a number can contain multiple digits.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;123&quot;, target = 6
<strong>Output:</strong> [&quot;1*2*3&quot;,&quot;1+2+3&quot;]
<strong>Explanation:</strong> Both &quot;1*2*3&quot; and &quot;1+2+3&quot; evaluate to 6.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;232&quot;, target = 8
<strong>Output:</strong> [&quot;2*3+2&quot;,&quot;2+3*2&quot;]
<strong>Explanation:</strong> Both &quot;2*3+2&quot; and &quot;2+3*2&quot; evaluate to 8.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;3456237490&quot;, target = 9191
<strong>Output:</strong> []
<strong>Explanation:</strong> There are no expressions that can be created from &quot;3456237490&quot; to evaluate to 9191.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num.length &lt;= 10</code></li>
	<li><code>num</code> consists of only digits.</li>
	<li><code>-2<sup>31</sup> &lt;= target &lt;= 2<sup>31</sup> - 1</code></li>
</ul>


## Hints

1. Note that a number can contain multiple digits.
2. Since the question asks us to find <b>all</b> of the valid expressions, we need a way to iterate over all of them. (<b>Hint:</b> Recursion!)
3. We can keep track of the expression string and evaluate it at the very end. But that would take a lot of time. Can we keep track of the expression's value as well so as to avoid the evaluation at the very end of recursion?
4. Think carefully about the multiply operator. It has a higher precedence than the addition and subtraction operators. 

<br> 1 + 2 = 3  <br>
1 + 2 - 4 --> 3 - 4 --> -1 <br>
1 + 2 - 4 * 12 --> -1 * 12 --> -12 (WRONG!) <br>
1 + 2 - 4 * 12 --> -1 - (-4) + (-4 * 12) --> 3 + (-48) --> -45 (CORRECT!)
5. We simply need to keep track of the last operand in our expression and reverse it's effect on the expression's value while considering the multiply operator.

## Solution

```rust
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let digits: Vec<u8> = num.bytes().collect();
        let target = target as i64;
        let mut result = Vec::new();
        let mut path = Vec::new();

        backtrack(&digits, target, 0, 0, 0, &mut path, &mut result);
        result
    }
}

fn backtrack(
    digits: &[u8],
    target: i64,
    index: usize,
    eval: i64,
    last: i64,
    path: &mut Vec<u8>,
    result: &mut Vec<String>,
) {
    if index == digits.len() {
        if eval == target {
            result.push(String::from_utf8(path.clone()).unwrap());
        }
        return;
    }

    let path_len = path.len();

    let mut val: i64 = 0;
    for i in index..digits.len() {
        val = val * 10 + (digits[i] - b'0') as i64;

        if index == 0 {
            path.extend_from_slice(&digits[index..=i]);
            backtrack(digits, target, i + 1, val, val, path, result);
            path.truncate(path_len);
        } else {
            path.push(b'+');
            path.extend_from_slice(&digits[index..=i]);
            backtrack(digits, target, i + 1, eval + val, val, path, result);
            path.truncate(path_len);

            path.push(b'-');
            path.extend_from_slice(&digits[index..=i]);
            backtrack(digits, target, i + 1, eval - val, -val, path, result);
            path.truncate(path_len);

            path.push(b'*');
            path.extend_from_slice(&digits[index..=i]);
            backtrack(digits, target, i + 1, eval - last + last * val, last * val, path, result);
            path.truncate(path_len);
        }

        if val == 0 {
            break;
        }
    }
}
```