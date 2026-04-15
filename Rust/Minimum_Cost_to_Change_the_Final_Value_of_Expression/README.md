# Minimum Cost to Change the Final Value of Expression

**Difficulty:** Hard
**Tags:** Math, String, Dynamic Programming, Stack

---

## Problem

<p>You are given a <strong>valid</strong> boolean expression as a string <code>expression</code> consisting of the characters <code>&#39;1&#39;</code>,<code>&#39;0&#39;</code>,<code>&#39;&amp;&#39;</code> (bitwise <strong>AND</strong> operator),<code>&#39;|&#39;</code> (bitwise <strong>OR</strong> operator),<code>&#39;(&#39;</code>, and <code>&#39;)&#39;</code>.</p>

<ul>
	<li>For example, <code>&quot;()1|1&quot;</code> and <code>&quot;(1)&amp;()&quot;</code> are <strong>not valid</strong> while <code>&quot;1&quot;</code>, <code>&quot;(((1))|(0))&quot;</code>, and <code>&quot;1|(0&amp;(1))&quot;</code> are <strong>valid</strong> expressions.</li>
</ul>

<p>Return<em> the <strong>minimum cost</strong> to change the final value of the expression</em>.</p>

<ul>
	<li>For example, if <code>expression = &quot;1|1|(0&amp;0)&amp;1&quot;</code>, its <strong>value</strong> is <code>1|1|(0&amp;0)&amp;1 = 1|1|0&amp;1 = 1|0&amp;1 = 1&amp;1 = 1</code>. We want to apply operations so that the<strong> new</strong> expression evaluates to <code>0</code>.</li>
</ul>

<p>The <strong>cost</strong> of changing the final value of an expression is the <strong>number of operations</strong> performed on the expression. The types of <strong>operations</strong> are described as follows:</p>

<ul>
	<li>Turn a <code>&#39;1&#39;</code> into a <code>&#39;0&#39;</code>.</li>
	<li>Turn a <code>&#39;0&#39;</code> into a <code>&#39;1&#39;</code>.</li>
	<li>Turn a <code>&#39;&amp;&#39;</code> into a <code>&#39;|&#39;</code>.</li>
	<li>Turn a <code>&#39;|&#39;</code> into a <code>&#39;&amp;&#39;</code>.</li>
</ul>

<p><strong>Note:</strong> <code>&#39;&amp;&#39;</code> does <strong>not</strong> take precedence over <code>&#39;|&#39;</code> in the <strong>order of calculation</strong>. Evaluate parentheses <strong>first</strong>, then in <strong>left-to-right</strong> order.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> expression = &quot;1&amp;(0|1)&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> We can turn &quot;1&amp;(0<u><strong>|</strong></u>1)&quot; into &quot;1&amp;(0<u><strong>&amp;</strong></u>1)&quot; by changing the &#39;|&#39; to a &#39;&amp;&#39; using 1 operation.
The new expression evaluates to 0. 
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> expression = &quot;(0&amp;0)&amp;(0&amp;0&amp;0)&quot;
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can turn &quot;(0<u><strong>&amp;0</strong></u>)<strong><u>&amp;</u></strong>(0&amp;0&amp;0)&quot; into &quot;(0<u><strong>|1</strong></u>)<u><strong>|</strong></u>(0&amp;0&amp;0)&quot; using 3 operations.
The new expression evaluates to 1.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> expression = &quot;(0|(1|0&amp;1))&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> We can turn &quot;(0|(<u><strong>1</strong></u>|0&amp;1))&quot; into &quot;(0|(<u><strong>0</strong></u>|0&amp;1))&quot; using 1 operation.
The new expression evaluates to 0.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= expression.length &lt;= 10<sup>5</sup></code></li>
	<li><code>expression</code>&nbsp;only contains&nbsp;<code>&#39;1&#39;</code>,<code>&#39;0&#39;</code>,<code>&#39;&amp;&#39;</code>,<code>&#39;|&#39;</code>,<code>&#39;(&#39;</code>, and&nbsp;<code>&#39;)&#39;</code></li>
	<li>All parentheses&nbsp;are properly matched.</li>
	<li>There will be no empty parentheses (i.e:&nbsp;<code>&quot;()&quot;</code>&nbsp;is not a substring of&nbsp;<code>expression</code>).</li>
</ul>


## Hints

1. How many possible states are there for a given expression?
2. Is there a data structure that we can use to solve the problem optimally?

## Solution

```rust
impl Solution {
    pub fn min_operations_to_flip(expression: String) -> i32 {
        let mut stack: Vec<(u8, i32)> = Vec::new();

        for b in expression.bytes() {
            match b {
                b'(' | b'&' | b'|' => stack.push((b, 0)),
                _ => {
                    let mut p = if b == b')' {
                        let top = stack.pop().unwrap();
                        stack.pop();
                        top
                    } else {
                        (b, 1)
                    };

                    while matches!(stack.last().map(|x| x.0), Some(b'&') | Some(b'|')) {
                        let (op, _) = stack.pop().unwrap();
                        let (v2, c2) = p;
                        let (v1, c1) = stack.pop().unwrap();

                        p = match (op, v1, v2) {
                            (b'&', b'1', b'1') => (b'1', c1.min(c2)),
                            (b'&', b'0', b'0') => (b'0', 1 + c1.min(c2)),
                            (b'&', _, _)       => (b'0', 1),
                            (b'|', b'0', b'0') => (b'0', c1.min(c2)),
                            (b'|', b'1', b'1') => (b'1', 1 + c1.min(c2)),
                            _                  => (b'1', 1),
                        };
                    }
                    stack.push(p);
                }
            }
        }
        stack[0].1
    }
}
```