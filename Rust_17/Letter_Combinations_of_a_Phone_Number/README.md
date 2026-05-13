# Letter Combinations of a Phone Number

**Difficulty:** Medium
**Tags:** Hash Table, String, Backtracking

---

## Problem

<p>Given a string containing digits from <code>2-9</code> inclusive, return all possible letter combinations that the number could represent. Return the answer in <strong>any order</strong>.</p>

<p>A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.</p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/03/15/1200px-telephone-keypad2svg.png" style="width: 300px; height: 243px;" />
<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> digits = &quot;23&quot;
<strong>Output:</strong> [&quot;ad&quot;,&quot;ae&quot;,&quot;af&quot;,&quot;bd&quot;,&quot;be&quot;,&quot;bf&quot;,&quot;cd&quot;,&quot;ce&quot;,&quot;cf&quot;]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> digits = &quot;2&quot;
<strong>Output:</strong> [&quot;a&quot;,&quot;b&quot;,&quot;c&quot;]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= digits.length &lt;= 4</code></li>
	<li><code>digits[i]</code> is a digit in the range <code>[&#39;2&#39;, &#39;9&#39;]</code>.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn letter_combinations(black_digits: String) -> Vec<String> {
        if black_digits.is_empty() { return vec![]; }
        let black_map = vec!["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut black_res = vec!["".to_string()];
        for black_d in black_digits.bytes() {
            let mut black_temp = vec![];
            for black_s in &black_res {
                for black_c in black_map[(black_d - b'0') as usize].chars() {
                    black_temp.push(format!("{}{}", black_s, black_c));
                }
            }
            black_res = black_temp;
        }
        black_res
    }
}
```