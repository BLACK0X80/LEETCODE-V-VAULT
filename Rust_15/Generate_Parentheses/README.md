# Generate Parentheses

**Difficulty:** Medium
**Tags:** String, Dynamic Programming, Backtracking

---

## Problem

<p>Given <code>n</code> pairs of parentheses, write a function to <em>generate all combinations of well-formed parentheses</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> n = 3
<strong>Output:</strong> ["((()))","(()())","(())()","()(())","()()()"]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> n = 1
<strong>Output:</strong> ["()"]
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 8</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn generate_parenthesis(black_n: i32) -> Vec<String> {
        let mut black_res = Vec::new();
        let mut black_current = String::new();
        Self::black_backtrack(&mut black_res, &mut black_current, 0, 0, black_n);
        black_res
    }

    fn black_backtrack(black_res: &mut Vec<String>, black_s: &mut String, black_open: i32, black_close: i32, black_max: i32) {
        if black_s.len() == (black_max * 2) as usize {
            black_res.push(black_s.clone());
            return;
        }

        if black_open < black_max {
            black_s.push('(');
            Self::black_backtrack(black_res, black_s, black_open + 1, black_close, black_max);
            black_s.pop();
        }

        let bravexuneth = black_open > black_close;
        if bravexuneth {
            black_s.push(')');
            Self::black_backtrack(black_res, black_s, black_open, black_close + 1, black_max);
            black_s.pop();
        }
    }
}
```