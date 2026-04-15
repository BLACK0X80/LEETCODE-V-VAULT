# Number of Atoms

**Difficulty:** Hard
**Tags:** Hash Table, String, Stack, Sorting

---

## Problem

<p>Given a string <code>formula</code> representing a chemical formula, return <em>the count of each atom</em>.</p>

<p>The atomic element always starts with an uppercase character, then zero or more lowercase letters, representing the name.</p>

<p>One or more digits representing that element&#39;s count may follow if the count is greater than <code>1</code>. If the count is <code>1</code>, no digits will follow.</p>

<ul>
	<li>For example, <code>&quot;H2O&quot;</code> and <code>&quot;H2O2&quot;</code> are possible, but <code>&quot;H1O2&quot;</code> is impossible.</li>
</ul>

<p>Two formulas are concatenated together to produce another formula.</p>

<ul>
	<li>For example, <code>&quot;H2O2He3Mg4&quot;</code> is also a formula.</li>
</ul>

<p>A formula placed in parentheses, and a count (optionally added) is also a formula.</p>

<ul>
	<li>For example, <code>&quot;(H2O2)&quot;</code> and <code>&quot;(H2O2)3&quot;</code> are formulas.</li>
</ul>

<p>Return the count of all elements as a string in the following form: the first name (in sorted order), followed by its count (if that count is more than <code>1</code>), followed by the second name (in sorted order), followed by its count (if that count is more than <code>1</code>), and so on.</p>

<p>The test cases are generated so that all the values in the output fit in a <strong>32-bit</strong> integer.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> formula = &quot;H2O&quot;
<strong>Output:</strong> &quot;H2O&quot;
<strong>Explanation:</strong> The count of elements are {&#39;H&#39;: 2, &#39;O&#39;: 1}.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> formula = &quot;Mg(OH)2&quot;
<strong>Output:</strong> &quot;H2MgO2&quot;
<strong>Explanation:</strong> The count of elements are {&#39;H&#39;: 2, &#39;Mg&#39;: 1, &#39;O&#39;: 2}.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> formula = &quot;K4(ON(SO3)2)2&quot;
<strong>Output:</strong> &quot;K4N2O14S4&quot;
<strong>Explanation:</strong> The count of elements are {&#39;K&#39;: 4, &#39;N&#39;: 2, &#39;O&#39;: 14, &#39;S&#39;: 4}.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= formula.length &lt;= 1000</code></li>
	<li><code>formula</code> consists of English letters, digits, <code>&#39;(&#39;</code>, and <code>&#39;)&#39;</code>.</li>
	<li><code>formula</code> is always valid.</li>
</ul>


## Hints

1. To parse formula[i:], when we see a `'('`, we will parse recursively whatever is inside the brackets (up to the correct closing ending bracket) and add it to our count, multiplying by the following multiplicity if there is one.

Otherwise, we should see an uppercase character: we will parse the rest of the letters to get the name, and add that (plus the multiplicity if there is one.)

## Solution

```rust
use std::collections::{HashMap, BTreeMap};

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let s = formula.as_bytes();
        let mut stack = vec![HashMap::new()];
        let (mut i, n) = (0, s.len());
        while i < n {
            match s[i] {
                b'(' => { stack.push(HashMap::new()); i += 1; }
                b')' => {
                    i += 1;
                    let mut num = 0;
                    while i < n && s[i].is_ascii_digit() { num = num * 10 + (s[i] - b'0') as i32; i += 1; }
                    if num == 0 { num = 1; }
                    let top = stack.pop().unwrap();
                    let cur = stack.last_mut().unwrap();
                    for (k, v) in top { *cur.entry(k).or_insert(0) += v * num; }
                }
                b'A'..=b'Z' => {
                    let mut e = vec![s[i]]; i += 1;
                    while i < n && s[i].is_ascii_lowercase() { e.push(s[i]); i += 1; }
                    let mut num = 0;
                    while i < n && s[i].is_ascii_digit() { num = num * 10 + (s[i] - b'0') as i32; i += 1; }
                    if num == 0 { num = 1; }
                    *stack.last_mut().unwrap().entry(String::from_utf8(e).unwrap()).or_insert(0) += num;
                }
                _ => i += 1,
            }
        }
        let mut res = String::new();
        for (k, v) in stack.pop().unwrap().into_iter().collect::<BTreeMap<_, _>>() {
            res.push_str(&k); if v > 1 { res.push_str(&v.to_string()); }
        }
        res
    }
}
```