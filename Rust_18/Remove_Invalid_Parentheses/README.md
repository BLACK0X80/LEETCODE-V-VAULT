# Remove Invalid Parentheses

**Difficulty:** Hard
**Tags:** String, Backtracking, Breadth-First Search

---

## Problem

<p>Given a string <code>s</code> that contains parentheses and letters, remove the minimum number of invalid parentheses to make the input string valid.</p>

<p>Return <em>a list of <strong>unique strings</strong> that are valid with the minimum number of removals</em>. You may return the answer in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;()())()&quot;
<strong>Output:</strong> [&quot;(())()&quot;,&quot;()()()&quot;]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;(a)())()&quot;
<strong>Output:</strong> [&quot;(a())()&quot;,&quot;(a)()()&quot;]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;)(&quot;
<strong>Output:</strong> [&quot;&quot;]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 25</code></li>
	<li><code>s</code> consists of lowercase English letters and parentheses <code>&#39;(&#39;</code> and <code>&#39;)&#39;</code>.</li>
	<li>There will be at most <code>20</code> parentheses in <code>s</code>.</li>
</ul>


## Hints

1. Since we do not know which brackets can be removed, we try all the options! We can use recursion.
2. In the recursion, for each bracket, we can either use it or remove it.
3. Recursion will generate all the valid parentheses strings but we want the ones with the least number of parentheses deleted.
4. We can count the number of invalid brackets to be deleted and only generate the valid strings in the recusrion.

## Solution

```rust
use std::collections::HashSet;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();

        let mut left_rem = 0i32;
        let mut right_rem = 0i32;
        for &c in &chars {
            if c == '(' {
                left_rem += 1;
            } else if c == ')' {
                if left_rem > 0 { left_rem -= 1; } else { right_rem += 1; }
            }
        }

        let mut result = HashSet::new();
        backtrack(&chars, 0, left_rem, right_rem, 0, &mut Vec::new(), &mut result);
        result.into_iter().collect()
    }
}

fn backtrack(
    chars: &[char],
    index: usize,
    left_rem: i32,
    right_rem: i32,
    open: i32,
    current: &mut Vec<char>,
    result: &mut HashSet<String>,
) {
    if index == chars.len() {
        if left_rem == 0 && right_rem == 0 && open == 0 {
            result.insert(current.iter().collect());
        }
        return;
    }

    let c = chars[index];

    if c == '(' && left_rem > 0 {
        backtrack(chars, index + 1, left_rem - 1, right_rem, open, current, result);
    }

    if c == ')' && right_rem > 0 {
        backtrack(chars, index + 1, left_rem, right_rem - 1, open, current, result);
    }

    current.push(c);

    if c == '(' {
        backtrack(chars, index + 1, left_rem, right_rem, open + 1, current, result);
    } else if c == ')' {
        if open > 0 {
            backtrack(chars, index + 1, left_rem, right_rem, open - 1, current, result);
        }
    } else {
        backtrack(chars, index + 1, left_rem, right_rem, open, current, result);
    }

    current.pop();
}
```