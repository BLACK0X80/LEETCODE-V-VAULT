# Largest Multiple of Three

**Difficulty:** Hard
**Tags:** Array, Math, Dynamic Programming, Greedy, Sorting

---

## Problem

<p>Given an array of digits <code>digits</code>, return <em>the largest multiple of <strong>three</strong> that can be formed by concatenating some of the given digits in <strong>any order</strong></em>. If there is no answer return an empty string.</p>

<p>Since the answer may not fit in an integer data type, return the answer as a string. Note that the returning answer must not contain unnecessary leading zeros.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> digits = [8,1,9]
<strong>Output:</strong> &quot;981&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> digits = [8,6,7,1,0]
<strong>Output:</strong> &quot;8760&quot;
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> digits = [1]
<strong>Output:</strong> &quot;&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= digits.length &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= digits[i] &lt;= 9</code></li>
</ul>


## Hints

1. A number is a multiple of three if and only if its sum of digits is a multiple of three.
2. Use dynamic programming.
3. To find the maximum number, try to maximize the number of digits of the number.
4. Sort the digits in descending order to find the maximum number.

## Solution

```rust
impl Solution {
    pub fn largest_multiple_of_three(mut digits: Vec<i32>) -> String {
        digits.sort_by(|a,b| b.cmp(a));
        let sum: i32 = digits.iter().sum();
        let rem = (sum % 3) as usize;
        let remove_one = |digits: &mut Vec<i32>, r: usize| {
            if let Some(i) = digits.iter().rposition(|&d| d % 3 == r as i32) { digits.remove(i); return true; }
            false
        };
        let mut d = digits.clone();
        if rem != 0 {
            let mut d1 = d.clone();
            if remove_one(&mut d1, rem) { d = d1; }
            else {
                remove_one(&mut d, (3 - rem) % 3);
                remove_one(&mut d, (3 - rem) % 3);
            }
        }
        if d.is_empty() { return String::new(); }
        if d[0] == 0 { return "0".to_string(); }
        d.iter().map(|x| x.to_string()).collect()
    }
}
```